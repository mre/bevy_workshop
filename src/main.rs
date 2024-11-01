use bevy::prelude::*;

mod game;
mod level_loader;
mod menu;
mod splash;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Workshop".into(),
                canvas: Some("#game".into()),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_plugins((
            splash::SplashPlugin,
            level_loader::LevelLoaderPlugin,
            menu::MenuPlugin,
            game::GamePlugin,
        ))
        .run();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Resource)]
struct GameAssets {
    player_image: Handle<Image>,
    player_layout: Handle<TextureAtlasLayout>,
    ground_image: Handle<Image>,
    ground_layout: Handle<TextureAtlasLayout>,
    items_image: Handle<Image>,
    items_layout: Handle<TextureAtlasLayout>,
}
