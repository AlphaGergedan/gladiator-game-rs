use bevy::prelude::*;

//use gladiator_game_rs::*;

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

pub fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let texture = asset_server.load("gladiator.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(128.0, 128.0), 2, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_indices = AnimationIndices { first: 0, last: 3 };

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
            SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                transform: Transform::from_scale(Vec3::splat(6.0)),
                ..default()
            }, animation_indices, AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));

    //commands.spawn(SpriteBundle {
        //texture: asset_server.load("gladiator.png"),
        //..default()
    //});
}

// RESOURCES

#[derive(Resource)]
pub struct GreetTimer(pub Timer);

// ANIMATIONS

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(time: Res<Time>, mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>) {
    for (indices, mut timer, mut atlas) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last { indices.first } else { atlas.index + 1 };
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevent blurry sprites
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, (setup_camera, add_game_players).chain())
        .add_systems(Update, (greet_game_player, animate_sprite))
        .run();
}

