use bevy::prelude::*;

// COMPONENTS

type StatValue_ = u64;

#[derive(Component)]
pub struct Stats {
    pub vitality: StatValue_,
    pub strength: StatValue_,
    pub defence: StatValue_,
    pub attack: StatValue_,
    pub agility: StatValue_,
    pub magic: StatValue_,
    pub stamina: StatValue_,
}

#[derive(Component)]
pub struct Gladiator {
    pub name: String,
    pub trivia: String,
    pub gender: String,
    pub stats: Stats,
}

#[derive(Component)]
pub struct GamePlayer;

#[derive(Component)]
pub struct GameEnemy_;

// SYSTEMS

pub fn greet_game_player(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Gladiator, With<GamePlayer>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for gladiator in query.iter() {
            println!("Hello Gladiator {}!", gladiator.name);
        }
    }
}

pub fn add_game_players(mut commands: Commands) {
    commands.spawn((GamePlayer, Gladiator { name: "John".to_string(), gender: "Male".to_string(), trivia: "omg".to_string(), stats: Stats { vitality: 100, strength: 100, defence: 100, attack: 100, agility: 100, magic: 100, stamina: 100 }}));
    commands.spawn((GamePlayer, Gladiator { name: "Michael".to_string(), gender: "Male".to_string(), trivia: "omg".to_string(), stats: Stats { vitality: 100, strength: 100, defence: 100, attack: 100, agility: 100, magic: 100, stamina: 100 }}));
    commands.spawn((GamePlayer, Gladiator { name: "Lucy".to_string(), gender: "Male".to_string(), trivia: "omg".to_string(), stats: Stats { vitality: 100, strength: 100, defence: 100, attack: 100, agility: 100, magic: 100, stamina: 100 }}));
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// RESOURCES

#[derive(Resource)]
pub struct GreetTimer(pub Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, (setup_camera, add_game_players).chain())
        .add_systems(Update, greet_game_player)
        .run();
}

