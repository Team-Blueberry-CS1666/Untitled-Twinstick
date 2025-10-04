use bevy::prelude::*;
use crate::GameState;

use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseButton;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // blank menu enter/exit logs
            .add_systems(OnEnter(GameState::Menu), on_enter_menu)
            .add_systems(OnExit(GameState::Menu), on_exit_menu)

            // Menu: start --> click / Enter / Space
            .add_systems(Update, start_on_input.run_if(in_state(GameState::Menu)))

            // Menu: quit --> Esc / Q / Backspace
            .add_systems(Update, quit_to_menu_on_input.run_if(in_state(GameState::Playing)))

            // optional logs on playing
            .add_systems(OnEnter(GameState::Playing), on_enter_playing)
            .add_systems(OnExit(GameState::Playing), on_exit_playing);
    }
}

fn on_enter_menu() {
    info!("STATE: MENU (blank). Click or press Enter/Space to START.");
}
fn on_exit_menu() {
    info!("Leaving MENU...");
}

fn on_enter_playing() {
    info!("STATE: PLAYING. Press Esc/Q/Backspace to QUIT to MENU.");
}
fn on_exit_playing() {
    info!("Leaving PLAYING → back to MENU.");
}

fn start_on_input(
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if mouse.just_pressed(MouseButton::Left)
        || keys.just_pressed(KeyCode::Enter)
        || keys.just_pressed(KeyCode::Space)
    {
        next_state.set(GameState::Playing);
    }
}

fn quit_to_menu_on_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape)
        || keys.just_pressed(KeyCode::KeyQ)
        || keys.just_pressed(KeyCode::Backspace)
    {
        next_state.set(GameState::Menu);
    }
}
