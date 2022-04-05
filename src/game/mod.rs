use crate::*;
use constants::AppState;
pub struct GamePlugin;

#[derive(Component)]
struct Score;

fn setup_in_game_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
) {
    let font = asset_server.load("fonts/Efforts.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto), // This will center the current node
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgba(1., 1., 1., 0.).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // Display two lines of text, the second one with the current settings
            parent.spawn_bundle(TextBundle {
                style: Style {
                    // margin: Rect::all(Val::Px(50.0)),
                    ..Default::default()
                },
                text: Text::with_section(
                    "Click W to win! Click Return to end.",
                    TextStyle {
                        font: font.clone(),
                        font_size: 64.0,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(32.0)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        format!("score: {}", game_state.score),
                        TextStyle {
                            font: font.clone(),
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(Score);

            // parent
            //     .spawn_bundle(ButtonBundle {
            //         style: Style {
            //             size: Size::new(Val::Px(160.0), Val::Px(65.0)),
            //             margin: Rect::all(Val::Auto),
            //             justify_content: JustifyContent::Center,
            //             align_items: AlignItems::Center,
            //             ..Default::default()
            //         },
            //         color: NORMAL_BUTTON.into(),
            //         ..Default::default()
            //     })
            //     .with_children(|parent| {
            //         parent.spawn_bundle(TextBundle {
            //             text: Text::with_section(
            //                 "try again",
            //                 TextStyle {
            //                     font: font.clone(),
            //                     font_size: 32.0,
            //                     color: Color::WHITE,
            //                 },
            //                 TextAlignment {
            //                     horizontal: HorizontalAlign::Right,
            //                     vertical: VerticalAlign::Center,
            //                 },
            //             ),
            //             style: Style {
            //                 margin: Rect {
            //                     left: Val::Px(0.),
            //                     right: Val::Px(0.),
            //                     top: Val::Px(0.),
            //                     bottom: Val::Px(10.),
            //                 },
            //                 ..Default::default()
            //             },
            //             ..Default::default()
            //         });
            //     });
        });
}

///
/// Update Main Menu
fn update_in_game_system(
    mut state: ResMut<State<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut query: Query<&mut Text, With<Score>>,
) {
    if keyboard_input.just_released(KeyCode::W) {
        game_state.score += 1;
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("score: {}", game_state.score);
        }
    }
    if keyboard_input.just_released(KeyCode::Return) {
        state.set(AppState::GameOver).unwrap();
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_in_game_system))
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(update_in_game_system),
            )
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(teardown_state));
    }
}
