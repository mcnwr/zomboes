use crate::components::Zombie;
use bevy::prelude::*;

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WaveState {
            current_wave: 1,
            zombies_remaining: 5,
            spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        })
        .add_systems(
            Update,
            wave_manager.run_if(in_state(crate::states::GameState::Playing)),
        );
    }
}

#[derive(Resource)]
pub struct WaveState {
    pub current_wave: u32,
    pub zombies_remaining: u32,
    pub spawn_timer: Timer,
}

fn wave_manager(
    mut commands: Commands,
    time: Res<Time>,
    mut wave_state: ResMut<WaveState>,
    zombies: Query<&Zombie>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
    settings: Res<crate::plugins::settings::GameSettings>,
) {
    // Difficulty is guaranteed to be set if we are in Playing state
    let difficulty = settings.difficulty.expect("Difficulty not set!");

    // Check if wave is cleared
    if wave_state.zombies_remaining == 0 && zombies.iter().count() == 0 {
        // Check Win Condition
        if wave_state.current_wave >= difficulty.max_waves() {
            info!("YOU WIN!");
            next_state.set(crate::states::GameState::Win);
            return;
        }

        // Start next wave
        wave_state.current_wave += 1;
        wave_state.zombies_remaining = 5 + wave_state.current_wave * 2;
        info!("Starting Wave {}", wave_state.current_wave);
    }

    // Spawn zombies over time
    wave_state.spawn_timer.tick(time.delta());
    if wave_state.spawn_timer.just_finished() && wave_state.zombies_remaining > 0 {
        let spawn_count = match difficulty {
            crate::plugins::settings::Difficulty::Easy => 1,
            crate::plugins::settings::Difficulty::Medium => 2,
            crate::plugins::settings::Difficulty::Hard => 3,
        };

        for _ in 0..spawn_count {
            if wave_state.zombies_remaining > 0 {
                spawn_random_zombie(&mut commands, wave_state.current_wave);
                wave_state.zombies_remaining -= 1;
            }
        }
    }
}

fn spawn_random_zombie(commands: &mut Commands, wave: u32) {
    let mut rng = rand::rng();
    use rand::Rng;

    let x = rng.random_range(-400.0..400.0);
    let y = rng.random_range(-300.0..300.0);

    // Zombie Size based on Level (Wave)
    // Level 1 = 10x10, Level 2 = 20x20, Level 3 = 30x30
    let (size_val, hp, reward) = if wave == 1 {
        (10.0, 10.0, 10)
    } else if wave == 2 {
        (20.0, 20.0, 20)
    } else {
        (30.0, 40.0, 30) // Level 3 and above
    };

    let size = Vec2::new(size_val, size_val);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.0, 0.0), // Red Zombie
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.5),
            ..default()
        },
        Zombie {
            money_reward: reward,
        },
        crate::components::Health {
            current: hp,
            max: hp,
        },
    ));
}
