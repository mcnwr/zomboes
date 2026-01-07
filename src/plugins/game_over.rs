use crate::states::GameState;
use bevy::prelude::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup_game_over)
            .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
            .add_systems(Update, restart_game.run_if(in_state(GameState::GameOver)))
            // Reuse Game Over logic for Win (for simplicity, or create separate)
            .add_systems(OnEnter(GameState::Win), setup_win)
            .add_systems(OnExit(GameState::Win), cleanup_game_over)
            .add_systems(Update, restart_game.run_if(in_state(GameState::Win)));
    }
}

#[derive(Component)]
struct GameOverUI;

fn setup_game_over(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
                ..default()
            },
            GameOverUI,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "GAME OVER",
                TextStyle {
                    font_size: 100.0,
                    color: Color::srgb(1.0, 0.0, 0.0),
                    ..default()
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Press SPACE for Menu",
                TextStyle {
                    font_size: 40.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            ));
        });
}

fn setup_win(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.5, 0.0, 0.8).into(), // Green Background
                ..default()
            },
            GameOverUI,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "MISSION COMPLETE!",
                TextStyle {
                    font_size: 80.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Press SPACE for Menu",
                TextStyle {
                    font_size: 40.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            ));
        });
}

fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn restart_game(
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
    mut wallet: ResMut<crate::plugins::shop::Wallet>,
    mut wave_state: ResMut<crate::plugins::wave::WaveState>,
    mut global_stats: ResMut<crate::plugins::progression::GlobalPlayerStats>,
) {
    if input.just_pressed(KeyCode::Space) {
        // Save Money to Global
        global_stats.total_money += wallet.money;

        // Reset In-Game Resources
        wallet.money = 0;
        *wave_state = crate::plugins::wave::WaveState {
            current_wave: 1,
            zombies_remaining: 10,
            spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        };

        // Transition to Dashboard
        // Cleanup of entities happens OnEnter(Dashboard)
        next_state.set(GameState::Dashboard);
    }
}
