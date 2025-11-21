use crate::{GameState, player::Player};
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

//NetControl gives the application access to the information sent by the clients
#[derive(Component)]
pub struct NetControl {
    pub host: bool,
    pub player_type: PlayerType,
    pub player_id: u8,

    pub net_input: u8,
    pub net_angle: u8,
    pub p_pos: Vec3,
    pub p_shot: bool,

    //Net
    player_addr: Option<SocketAddr>,
    player_pos_x: i32,
    player_pos_y: i32,
}
impl NetControl {
    pub fn new(is_host: bool, ptype: PlayerType, pid: u8, addr: Option<SocketAddr>) -> Self {
        Self {
            host: is_host,
            //I have added single components for both Local and Network, to allow them to go directly on the player
            player_type: ptype,
            player_id: pid,

            net_input: 0,
            net_angle: 0,
            p_pos: Vec3::ZERO,
            p_shot: false,

            player_addr: addr,
            player_pos_x: 0,
            player_pos_y: 0,
        }
    }

    //Getter for player_type
    pub fn get_type(&self) -> PlayerType {
        return self.player_type;
    }

    //Getter for player_addr
    pub fn get_addr(&self) -> Option<SocketAddr> {
        return self.player_addr;
    }

    //Getter for p_pos
    pub fn get_p_pos(&self) -> Vec3 {
        return self.p_pos;
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ///      Inputs from remote player Functions --> We have 5/8 inputs, so up to 3 can be added without     ///
    ///                                 me having to restructure our packets.                                ///
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////

    //Functions like KeyCode.pressed, but for the networked user. WASD is currently supported.
    pub fn pressed(&self, input: KeyCode) -> bool {
        match (input) {
            KeyCode::KeyW => self.net_input & 128 == 128,
            KeyCode::KeyA => self.net_input & 64 == 64,
            KeyCode::KeyS => self.net_input & 32 == 32,
            KeyCode::KeyD => self.net_input & 16 == 16,
            _ => false,
        }
    }

    //Functions like MouseButton.pressed, but for the networked user. Only Left Click is currently supported.
    pub fn clicked(&self, input: MouseButton) -> bool {
        match (input) {
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
    pub fn get_angle(&self) -> f32 {
        //This assumes that you've already rounded the float to 1 decimal point
        let angle_as_i8 = (self.net_angle as i8);
        return (angle_as_i8 as f32) / 10.;
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ///                                         Position functions                                           ///
    ///                                                                                                      ///
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////

    //Setter for pos_x
    pub fn set_pos_x(&mut self, x_pos: f32) {
        self.player_pos_x = x_pos as i32;
    }

    //Setter for pos_y
    pub fn set_pos_y(&mut self, y_pos: f32) {
        self.player_pos_y = y_pos as i32;
    }

    //MAY RUN INTO ENDIAN ISSUES BETWEEN DIFFERENT COMPUTERS
    //Getter for pos_x
    pub fn get_pos_x(&self) -> [u8; 4] {
        return self.player_pos_x.to_ne_bytes();
    }

    //Getter for pos_y
    pub fn get_pos_y(&self) -> [u8; 4] {
        return self.player_pos_y.to_ne_bytes();
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ///                                     Outgoing Server Packet                                           ///
    ///                                                                                                      ///
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn get_out_packet(&self, op: u8, pid: u8) -> [u8; 11] {
        let mut out_pack: [u8; 11] = [0; 11];
        let out_x = self.get_pos_x();
        let out_y = self.get_pos_y();
        out_pack[0] = op;
        //I'm packing p_shot into the player_id byte and no one can stop me
        out_pack[1] = ((self.net_input & 3) << 6) + pid;
        out_pack[2..6].copy_from_slice(&out_x);
        out_pack[6..10].copy_from_slice(&out_y);
        out_pack[10] = self.net_angle;
        return out_pack;
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ///                                     Incoming Client Packet                                           ///
    ///                                                                                                      ///
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn set_player_state(&mut self, pack: [u8; 11]) {
        let mut unpack_x: [u8; 4] = [0; 4];
        let mut unpack_y: [u8; 4] = [0; 4];
        unpack_x.copy_from_slice(&pack[2..6]);
        unpack_y.copy_from_slice(&pack[6..10]);
        let x = i32::from_ne_bytes(unpack_x);
        let y = i32::from_ne_bytes(unpack_y);
        self.p_pos = Vec3::new(x as f32, y as f32, 0.);
        self.p_shot = { pack[1] & 128 == 128 };
        self.net_angle = pack[10];
        //info!("Player {}'s Position: {:?}", self.player_id, self.p_pos);
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum PlayerType {
    Local,
    Network,
}

#[derive(Component)]
pub struct Local;

#[derive(Component)]
pub struct Network;

pub struct NetControlPlugin;
impl Plugin for NetControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Lobby), net_control_init);
    }
}

fn net_control_init(mut commands: Commands) {
    /*commands.spawn((
        NetControl::new(PlayerType::Local, 0, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 2525)),
    ));*/
}
