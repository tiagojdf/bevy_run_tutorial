use crate::*;
use constants::AppState;
pub struct MainMenuPlugin;

/**
 * Adding a button
 */
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn handle_user_interactions(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn setup_menu_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Headline
    commands.spawn_bundle(TextBundle {
        /// Describes the style including flexbox settings
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(4.0),
                left: Val::Px(24.0),
                ..Default::default()
            },
            ..Default::default()
        },
        /// Contains the text of the node
        text: Text::with_section(
            format!("Run in Rust"),
            TextStyle {
                font: asset_server.load("fonts/Efforts.ttf"),
                font_size: 64.0,
                color: Color::WHITE,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                vertical: VerticalAlign::Center,
            },
        ),
        ..Default::default()
    });
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Start",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu_system))
            .add_system_set(
                SystemSet::on_update(AppState::Menu).with_system(handle_user_interactions),
            )
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(teardown_state));
    }
}
