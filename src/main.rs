use bevy::prelude::*;

mod constants;
use constants::*;

///
/// State Plugins
mod main_menu;
use main_menu::MainMenuPlugin;
mod game;
use game::GamePlugin;
mod game_over;
use game_over::GameOverPlugin;

// Debug
#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

// Game State => resource to hold score information
#[derive(Default)]
pub struct GameState {
    pub score: u64,
}

fn camera_setup(mut commands: Commands) {
    // 2D ortho camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    //
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn teardown_state(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn main() {
    let mut app = App::new();
    // windwo setup
    app.insert_resource(ClearColor(BG_COLOR))
        .insert_resource(WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: "Run Rust!".to_string(),
            vsync: true,
            ..Default::default()
        })
        .insert_resource(GameState { score: 0 })
        .add_plugins(DefaultPlugins)
        // States
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GameOverPlugin)
        //TODO add other states
        .add_state(AppState::Menu); // This is the state we start in!

    // Debug
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_startup_system(camera_setup);
    app.run();
}
