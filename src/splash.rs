use bevy::prelude::*;
use crate::{despawn_screen, MainState};

// Tag component used to tag entities added on splash screen
#[derive(Component)]
struct OnSplashScreen;

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("-> Setting up startup splash");
    let icon = asset_server.load("spider.png");
    commands
        .spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    width: Val::Px(200.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        });

    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

// display splash plugin logo before entering game menu
pub fn splash_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(MainState::Splash), splash_setup)
        .add_systems(Update, countdown.run_if(in_state(MainState::Splash)))
        .add_systems(OnExit(MainState::Splash), despawn_screen::<OnSplashScreen>);
    }

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

// transition to game menu after splash screen
fn countdown(mut game_state: ResMut<NextState<MainState>>, time: Res<Time>, mut timer: ResMut<SplashTimer>) {
    if timer.tick(time.delta()).finished() {
        game_state.set(MainState::Menu);
    }
}
