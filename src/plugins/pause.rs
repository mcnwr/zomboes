use crate::states::GameState;
use bevy::prelude::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_pause)
            .add_systems(OnEnter(GameState::Paused), setup_pause_menu)
            .add_systems(OnExit(GameState::Paused), cleanup_pause_menu);
    }
}

#[derive(Component)]
struct PauseUI;

fn toggle_pause(
    input: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        match state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }

    // ESC to Main Menu
    if input.just_pressed(KeyCode::Escape)
        && (*state.get() == GameState::Playing || *state.get() == GameState::Paused)
    {
        next_state.set(GameState::Dashboard);
    }
}

fn setup_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.5).into(),
                ..default()
            },
            PauseUI,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "PAUSED",
                TextStyle {
                    font_size: 80.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

fn cleanup_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseUI>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
