use crate::{GameState, player::Player,};
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use std::net::{UdpSocket, IpAddr, Ipv4Addr, SocketAddr};

//NetControl gives the application access to the information sent by the clients
#[derive(Component)]
pub struct NetControl {
    pub player_type: PlayerType,
    pub net_input: u8,
    pub net_angle: u8,
    pub player_id: u8,
    player_addr: SocketAddr,
}
impl NetControl {
    pub fn new(ptype: PlayerType, pid: u8, addr: SocketAddr) -> Self {
        Self {
            player_type: ptype,
            net_input: 0,
            net_angle: 0,
            player_id: pid,
            player_addr: addr,
        }
    }

    //Getter for player_type
    pub fn get_type(&self) -> PlayerType {
        return self.player_type;
    }

    //Getter for player_addr
    pub fn get_addr(&self) -> SocketAddr {
        return self.player_addr;
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ///      Inputs from remote player Functions --> We have 5/8 inputs, so up to 3 can be added without     ///
    ///                                 me having to restructure our packets.                                ///
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////

    //Functions like KeyCode.pressed, but for the networked user. WASD is currently supported.
    pub fn pressed(&self, input: KeyCode) -> bool {
        match(input) {
            KeyCode::KeyW => self.net_input & 128 == 128,
            KeyCode::KeyA => self.net_input & 64 == 64,
            KeyCode::KeyS => self.net_input & 32 == 32,
            KeyCode::KeyD => self.net_input & 16 == 16,
            _ => false,
        } 
    }

    //Functions like MouseButton.pressed, but for the networked user. Only Left Click is currently supported.
    pub fn clicked(&self, input: MouseButton) -> bool {
        match(input) {
            MouseButton::Left => self.net_input & 2 == 2,
            _ => false,
        } 
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ///                                            Angle functions                                           ///
    ///                                                                                                      ///
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////

    //Setter for net_angle
    pub fn set_angle(&mut self, angle: f32) {
        //This assumes that you've already rounded the float to 1 decimal point
        let angle_as_i8 = (angle * 10.) as i8;
        self.net_angle = angle_as_i8 as u8;
    }

    //Getter for net_angle
    pub fn get_angle(&self) -> f32{
        //This assumes that you've already rounded the float to 1 decimal point
        let angle_as_i8 = (self.net_angle as i8);
        return angle_as_i8 as f32;
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
}

#[derive(Clone, Copy, PartialEq)]
pub enum PlayerType {
    Local,
    Network,
}

pub struct NetControlPlugin;
impl Plugin for NetControlPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Lobby), net_control_init);
    }
}

fn net_control_init(
    mut commands: Commands,
) {
    /*commands.spawn((
        NetControl::new(PlayerType::Local, 0, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 2525)),
    ));*/
}