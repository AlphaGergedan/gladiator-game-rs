use bevy::prelude::*;

const TEXT_COLOR: Color = Color::GRAY;
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MainState {
    #[default]
    Splash,

    Menu,
    Game,
}

mod splash;
mod menu;
mod game;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low, Medium, High
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u8);

fn setup_camera(mut commands: Commands) {
    println!("-> Setting up main camera 2d bundle once at main.rs");
    commands.spawn(Camera2dBundle::default());
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn main() {
    println!("-> Creating bevy 'App'");
    App::new()
        .add_plugins(DefaultPlugins)
        // Insert as resource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(5))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<MainState>()
        .add_systems(Startup, setup_camera)
        // Adds the plugins for each state
        //.add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin)) # FIXME: after implementing game plugin uncomment this line and delete the next line
        .add_plugins((splash::splash_plugin, menu::menu_plugin))
        .run();
}
