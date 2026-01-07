use bevy::prelude::*;
use crate::components::Zombie;
use crate::plugins::player::Player;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_zombies)
           .add_systems(Update, zombie_chase.run_if(in_state(crate::states::GameState::Playing)));
    }
}

fn spawn_initial_zombies(mut commands: Commands) {
    // Spawn a few testing zombies
    let positions = vec![
        Vec3::new(200.0, 200.0, 0.5),
        Vec3::new(-200.0, 200.0, 0.5),
        Vec3::new(200.0, -200.0, 0.5),
        Vec3::new(-200.0, -200.0, 0.5),
    ];

    for pos in positions {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(1.0, 0.0, 0.0), // Red Zombie
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_translation(pos),
                ..default()
            },
            Zombie { money_reward: 10 },
            crate::components::Health { current: 10.0, max: 10.0 },
        ));
    }
}

fn zombie_chase(
    player_query: Query<&Transform, With<Player>>,
    mut zombie_query: Query<(&mut Transform, &Sprite), (With<Zombie>, Without<Player>)>,
    wall_query: Query<(&Transform, &crate::components::Collider), (With<crate::components::Wall>, Without<Zombie>)>,
    time: Res<Time>,
) {
    let speed = 80.0;
    if let Ok(player_transform) = player_query.get_single() {
        for (mut zombie_transform, sprite) in &mut zombie_query {
            let direction = (player_transform.translation - zombie_transform.translation).normalize_or_zero();
            let proposed = zombie_transform.translation + direction * speed * time.delta_seconds();
            
            let zombie_size = sprite.custom_size.unwrap_or(Vec2::new(32.0, 32.0));
            let mut collision = false;
            
            for (w_transform, w_collider) in &wall_query {
                 let z_min = proposed.truncate() - zombie_size / 2.0;
                 let z_max = proposed.truncate() + zombie_size / 2.0;
                 let w_min = w_transform.translation.truncate() - w_collider.size / 2.0;
                 let w_max = w_transform.translation.truncate() + w_collider.size / 2.0;

                 if z_min.x < w_max.x && z_max.x > w_min.x &&
                    z_min.y < w_max.y && z_max.y > w_min.y {
                     collision = true;
                     break;
                 }
            }

            if !collision {
                zombie_transform.translation = proposed;
            }
        }
    }
}
