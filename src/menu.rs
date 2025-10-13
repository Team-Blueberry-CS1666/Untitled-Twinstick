use crate::GameState;
use bevy::prelude::*;

use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;

#[derive(Component)]
enum MenuButton {
    Host,
    Join,
    Credits,
    Exit,
}

// tags UI elements for cleanup
#[derive(Component)]
struct MenuUI;

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
            .add_systems(
                Update,
                quit_to_menu_on_input.run_if(in_state(GameState::Playing)),
            )
            // optional logs on playing
            .add_systems(OnEnter(GameState::Playing), on_enter_playing)
            .add_systems(OnExit(GameState::Playing), on_exit_playing);
    }
}

fn on_enter_menu(mut commands: Commands) {
    info!("STATE: MENU (blank). Click or press Enter/Space to START.");
    commands.spawn((Camera2d, MenuUI));

    // creates a UI containter that fills the 100% of the window (width and height)
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            MenuUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Untitled Twinstick Shooter"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            // container for buttons
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|parent| {
                    // host button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(50.0),
                                border: UiRect::all(Val::Px(2.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BorderColor(Color::WHITE),
                            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                            MenuButton::Host,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("HOST"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // join button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(50.0),
                                border: UiRect::all(Val::Px(2.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BorderColor(Color::WHITE),
                            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                            MenuButton::Join,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("JOIN"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // credits button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(50.0),
                                border: UiRect::all(Val::Px(2.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BorderColor(Color::WHITE),
                            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                            MenuButton::Credits,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("CREDITS"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // exit button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(50.0),
                                border: UiRect::all(Val::Px(2.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BorderColor(Color::WHITE),
                            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                            MenuButton::Exit,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("EXIT"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });
}

fn on_exit_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    info!("Leaving MENU...");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn on_enter_playing() {
    info!("STATE: PLAYING. Press Esc/Q/Backspace to QUIT to MENU.");
}
fn on_exit_playing() {
    info!("Leaving PLAYING → back to MENU.");
}

fn start_on_input(
    mut interaction_query: Query<
        (&Interaction, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    for (interaction, menu_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button {
                MenuButton::Host => {
                    info!("host button pressed.");
                    next_state.set(GameState::Playing);
                }
                MenuButton::Join => {
                    info!("join button pressed.");
                }
                MenuButton::Credits => {
                    info!("credits button pressed.");
                    next_state.set(GameState::Credits);
                }
                MenuButton::Exit => {
                    info!("exit button pressed.");
                    exit.write(bevy::app::AppExit::Success);
                }
            }
        }
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