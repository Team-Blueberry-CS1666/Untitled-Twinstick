use crate::{
    AssignedType, GameState, LogicType, net_control::NetControl, net_control::PlayerType, net_control::Local, net_control::Network,
    player::Player,
    collectible::PlayerInventory,
};
use bevy::prelude::*;
use std::net::UdpSocket;

const IP_CONST: &str = "127.0.0.1:";

#[derive(Resource)]
pub struct SocketResource {
    socket: UdpSocket,
}

pub struct ClientPlugin;
impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Joining),
            (
                client_init.before(client_start),
                client_start.before(client_connect),
                client_connect,
            ),
        )
        .add_systems(
            FixedUpdate,
            client_run
                .run_if(in_state(GameState::Joining))
                .run_if(type_equals_client),
        )
        .add_systems(
            FixedUpdate,
            client_run
                .run_if(in_state(GameState::Playing))
                .run_if(type_equals_client),
        )
        .add_systems(
            FixedUpdate,
            input_converter
                .run_if(in_state(GameState::Playing))
                .run_if(type_equals_client),
        )
        .add_systems(
            OnExit(GameState::Playing),
            client_close.run_if(type_equals_client),
        );
    }
}

fn type_equals_client(game_type: Res<LogicType>) -> bool {
    return game_type.l_type == AssignedType::Client;
}

fn client_init(mut commands: Commands) {
    info!("In client init");
    //The random port is so I can hard code it and run more than one client at a time.
    let newIP = IP_CONST.to_owned() + &rand::random_range(25000..25999).to_string();
    commands.insert_resource(SocketResource {
        socket: UdpSocket::bind(newIP).expect("ERROR"),
    });
}

fn client_start(socket: ResMut<SocketResource>) {
    info!("In client start");
    //This makes it so the game doesn't wait to receive a message, before going to the next frame
    socket.socket.set_nonblocking(true);
}

fn client_close(mut commands: Commands) {
    commands.remove_resource::<SocketResource>();
}

fn client_connect(socket: ResMut<SocketResource>) {
    info!("In client connect");
    let mut buf = [0];
    socket
        .socket
        .send_to(&[255], "127.0.0.1:2525")
        .expect("couldn't send data");
    match socket.socket.recv_from(&mut buf) {
        Ok((amt, src)) => {
            info!("{:?} + {:?} + {:?}", amt, src, buf);
        }
        Err(e) => {
            //info!("{:?}", e);
        }
    }
}

fn client_run(
    mut commands: Commands,
    socket: ResMut<'_, SocketResource>,
    mut p_loc: Query<&mut NetControl, With<NetControl>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut buf = [0; 11];

    for i in 1..8 {
        match socket.socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                //info!("{:?} + {:?} + {:?}", amt, src, buf);
                match buf[0] {
                    //Code 0 -> Game Started. Send player counts for NetControl initialization.
                    0 => {
                        for i in 0..buf[1] {
                            if i == buf[2] {
                                commands.spawn(
                                    (NetControl::new(false, PlayerType::Local, i, None),
                                    Local,
                                )
                            );
                                info!("I am player: {}", i);
                            } else {
                                commands.spawn((NetControl::new(
                                    false,
                                    PlayerType::Network,
                                    i,
                                    None,
                                ),
                                Network,
                                )
                            );
                                info!("Created net player: {}", i);
                            }
                        }
                        //Start the game
                        info!("PLAY STATE");
                        next_state.set(GameState::Playing);
                    }

                    //Code 1 -> Player position/angle update.
                    1 => {
                        for mut i in p_loc.iter_mut() {
                            //The first check hard limits us to 4 players (pid 0 to 3) as I started packing shooting into the same byte.
                            //The second check prevents server from overwriting active player info. Will need to add 'else' to handle rollback system
                            if i.player_id == (buf[1] & 3) && i.player_type == PlayerType::Network {
                                i.set_player_state(buf);
                            }
                        }
                    }
                    _ => {
                        info!("{:?} + {:?} + {:?}", amt, src, buf);
                    }
                }
            }
            Err(e) => {
                //info!("ERROR");
            }
        }
    }
}

pub fn input_converter(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mouse_button_io: Res<ButtonInput<MouseButton>>,
    socket: ResMut<SocketResource>,
    mut pl_cont: Query<&mut NetControl, (With<NetControl>, With<Local>)>,
    mut inventory: ResMut<PlayerInventory>,
)   {
        let mut input_result: u8 = 0;
        let mut player = pl_cont.single_mut().unwrap();
        //WASD00L0
        if input.pressed(KeyCode::KeyW) {
            input_result += 128;
        }

        if input.pressed(KeyCode::KeyA) {
            input_result += 64;
        }

        if input.pressed(KeyCode::KeyS) {
            input_result += 32;
        }

        if input.pressed(KeyCode::KeyD) {
            input_result += 16;
        }

        //Ideally we would still send the click signal always, but this is easier if we aren't sending ammo info
        if mouse_button_io.pressed(MouseButton::Left) && inventory.has_available_ammo() {
            input_result += 2;
        }

        socket
            .socket
            .send_to(&[input_result, player.net_angle], "127.0.0.1:2525")
            .expect("couldn't send data");

    //info!("WASD");
    //info!("{:08b}", input_result);

    /*for i in input.get_pressed() {
        info!("(KEYBOARD) {:?} is pressed.", i);
    }

    for i in mouse_button_io.get_pressed(){
        info!("(MOUSE) {:?} is pressed.", i);
    }*/
}
