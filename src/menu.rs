use bevy::{app::AppExit, prelude::*};
use crate::{despawn_screen, MainState, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON, TEXT_COLOR};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    #[default]
    Disabled,

    Main,
    NewGame,
    LoadGame,
    Settings,
}

mod main;
mod new_game;
mod load_game;
mod settings;

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct OnNewGameMenuScreen;

#[derive(Component)]
struct OnLoadGameMenuScreen;

#[derive(Component)]
struct OnSettingsMenuScreen;

#[derive(Component)]
enum MenuButtonAction {
    CreateNewGame,
    LoadSavedGame,
    BackToMainMenu,
    OpenSettings,
    Quit,
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(20.0)),
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        // cover full screen
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                // flex container
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::DARK_GRAY.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // game title
                    parent.spawn(
                        TextBundle::from_section(
                            "Gladiator Game",
                            TextStyle {
                                font_size: 80.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        ).with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                })

                // Display three buttons
                // - new game
                // - load game
                // - settings
                // - quit
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::CreateNewGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("New Game", button_text_style.clone()));
                        });
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::LoadSavedGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Load Game", button_text_style.clone()));
                        });
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::OpenSettings,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Settings", button_text_style.clone()));
                        });
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Quit", button_text_style.clone()));
                        });
                });
        });
}

fn new_game_menu_setup() { todo!(); }
fn load_game_menu_setup() { todo!(); }
fn settings_menu_setup() { todo!(); }

fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<MainState>>
)
{
    for (&interaction, menu_button_action) in &interaction_query {
        if interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::CreateNewGame => {
                    println!("-> Create new game button pressed!");
                },
                MenuButtonAction::LoadSavedGame => {
                    println!("-> Load saved game button pressed!");
                },
                MenuButtonAction::BackToMainMenu => {
                    println!("-> Back to main menu button pressed!");
                },
                MenuButtonAction::OpenSettings => {
                    println!("-> Open settings button pressed!");
                },
                MenuButtonAction::Quit => {
                    println!("-> Quit button pressed!");
                }
            }
        }
    }
}

fn button_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>
) {
    for (&interaction, mut color) in &mut interaction_query {
        *color = match interaction {
            Interaction::Pressed => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        }
    }
}

pub fn menu_plugin(app: &mut App) {
    app
        .init_state::<MenuState>()
        .add_systems(OnEnter(MainState::Menu), menu_setup)
        // setup and destruct for each screen
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        .add_systems(OnEnter(MenuState::NewGame), new_game_menu_setup)
        .add_systems(OnExit(MenuState::NewGame), despawn_screen::<OnNewGameMenuScreen>)
        .add_systems(OnEnter(MenuState::LoadGame), load_game_menu_setup)
        .add_systems(OnExit(MenuState::LoadGame), despawn_screen::<OnLoadGameMenuScreen>)
        .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
        .add_systems(OnExit(MenuState::Settings), despawn_screen::<OnSettingsMenuScreen>)
        // buttons
        .add_systems(Update, (menu_action, button_system).run_if(in_state(MainState::Menu)));
}
