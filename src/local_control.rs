use crate::{GameState, player::Player,};
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use std::net::{UdpSocket, IpAddr, Ipv4Addr, SocketAddr};

//LocalControl gives the application access to the information sent by the clients
#[derive(Component)]
pub struct LocalControl {
    pub player_type: PlayerType,
    pub p_pos: Vec2,
    pub p_mov: Vec2,
    pub p_angle: u8,
    pub player_id: u8,
    //player_addr: SocketAddr,
}
impl LocalControl {
    pub fn new(ptype: PlayerType, pid: u8) -> Self {
        Self {
            player_type: ptype,
            p_input: 0,
            p_angle: 0,
            player_id: pid,
        }
    }

    //Getter for player_type
    pub fn get_type(&self) -> PlayerType {
        return self.player_type;
    }

    //Getter for p_angle
    pub fn get_angle(&self) -> f32{
        //This assumes that you've already rounded the float to 1 decimal point
        let angle_as_i8 = (self.p_angle as i8);
        return angle_as_i8 as f32;
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
}

#[derive(Clone, Copy, PartialEq)]
pub enum PlayerType {
    Local,
    Network,
}

pub struct LocalControlPlugin;
impl Plugin for LocalControlPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Lobby), local_control_init);
    }
}

fn local_control_init(
    mut commands: Commands,
) {
    /*commands.spawn((
        NetControl::new(PlayerType::Local, 0, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 2525)),
    ));*/
}