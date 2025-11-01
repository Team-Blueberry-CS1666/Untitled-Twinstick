use crate::{GameState, player::Player, net_control::NetControl, net_control::PlayerType,};
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use std::net::{UdpSocket, IpAddr, Ipv4Addr, SocketAddr};
use std::thread;

const IP_CONST: &str = "127.0.0.1:2525";
const MAX_PLAYER: u8 = 4;


#[derive(Resource)]
pub struct SocketResource {
    socket: UdpSocket,
}

#[derive(Resource)]
pub struct inFlag {
    pub ready: bool,
}

#[derive(Resource)]
pub struct player_count {
    pub count: u8,
}

pub struct ServerPlugin;
impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            OnEnter(GameState::Lobby),
            (server_init.before(server_start), server_start),
        )
        .add_systems(
            OnEnter(GameState::Playing),
            (send_players),
        )
        .add_systems(Update, server_run.run_if(in_state(GameState::Lobby)))
        .add_systems(Update, server_run.run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(GameState::Playing), server_close);
    }
}

fn server_init(mut commands: Commands) {
    commands.insert_resource(SocketResource {
        socket: UdpSocket::bind(IP_CONST).expect("ERROR"),
    });
    commands.insert_resource(inFlag { ready: false });
    commands.insert_resource(player_count { count: 1 });
    commands.spawn((
        NetControl::new(PlayerType::Local, 0, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 2525)),
    ));
}

fn server_close(mut commands: Commands) {
    commands.remove_resource::<SocketResource>();
}

fn server_start(socket: ResMut<SocketResource>) {
    //This makes it so the game doesn't wait to receive a message, before going to the next frame
    socket.socket.set_nonblocking(true);
}

fn server_run(
    mut commands: Commands,
    socket: ResMut<'_, SocketResource>,
    mut player: Query<&mut NetControl, With<NetControl>>,
    mut flag: ResMut<inFlag>,
    mut count: ResMut<player_count>,
) {
    let mut buf = [0; 10];
    for i in 1..4 {
        match socket.socket.recv_from(&mut buf) {
            Ok((amt, src)) => {

                //Server Receives Join Packet
                if (buf[0] == 255) {
                    //info!("{:?} + {:?} + {:?}", src, amt, buf);
                    if count.count < MAX_PLAYER {
                        let tempNet = commands.spawn((
                            NetControl::new(PlayerType::Network, count.count, src)
                        )).id();
                        count.count += 1;
                    }
                    else
                    {
                        //Send Reject Message
                    }
                }

                //Input Byte
                
                for mut a in player.iter_mut() {
                    if a.get_addr() == src {
                        a.net_input = buf[0];
                        a.net_angle = buf[1];
                    }
                }

                /*if map.map.contains_key(&src) {
                    match player.get_mut(*map.map.get(&src).unwrap()).unwrap() {
                        Ok(i) => {
                            i.net_input = buf[0];
                            i.net_angle = buf[1];
                        }
                        Err(e) => {
                            info!("{:?}", e)
                        }
                    }
                }*/

                /*socket
                    .socket
                    .send_to(&[1; 10], src)
                    .expect("couldn't send data");*/
            }
            Err(e) => {
                //info!("Nothing");
            }
        }
    }
}

fn send_players(
    socket: ResMut<'_, SocketResource>,
    mut player: Query<&mut NetControl, With<NetControl>>,
    count: ResMut<player_count>,
) {
    for i in player {
        if i.get_type() == PlayerType::Network {
            //info!{"{:?}", i.get_addr()};
            socket
                    .socket
                    .send_to(&[count.count], i.get_addr())
                    .expect("couldn't send data");
        }
    }
}
