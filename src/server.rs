use bevy::prelude::*;
use crate::GameState;
use std::net::UdpSocket;

const IP_CONST: &str = "127.0.0.1:2525";

#[derive(Resource)]
pub struct SocketResource {
    socket: UdpSocket,
}

pub struct ServerPlugin;
impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), (server_init.before(server_start), server_start))
            .add_systems(FixedUpdate, server_run.run_if(in_state(GameState::Playing)));
    }
}

fn server_init(
    mut commands: Commands,
) {
    commands.insert_resource(SocketResource{socket: UdpSocket::bind(IP_CONST).expect("ERROR")});
}

fn server_start(
    commands: Commands,
    mut socket: ResMut<SocketResource>,
) {
    //This makes it so the game doesn't wait to receive a message, before going to the next frame
    socket.socket.set_nonblocking(true);
}

fn server_run(
    mut socket: ResMut<'_, SocketResource>,
) {
    let mut buf = [0; 10];
    
    //This might only work for one client at a time, so we may need to adjust this when we get further
    match socket.socket.recv_from(&mut buf)
    {
        Ok((amt, src)) => {
            info!("{:?} + {:?} + {:?}", amt, src, buf);
        }
        Err(e) => {
            //info!("ERROR");
        }
    }
}