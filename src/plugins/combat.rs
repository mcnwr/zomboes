use crate::components::Projectile;
use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                projectile_movement,
                cleanup_projectiles,
                projectile_collision,
                zombie_damage_player,
            )
                .run_if(in_state(crate::states::GameState::Playing)),
        );
    }
}

fn projectile_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Projectile)>) {
    for (mut transform, projectile) in &mut query {
        transform.translation.x += projectile.velocity.x * time.delta_seconds();
        transform.translation.y += projectile.velocity.y * time.delta_seconds();
    }
}

fn projectile_collision(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform, &Projectile), With<Projectile>>,
    mut zombie_query: Query<
        (
            Entity,
            &Transform,
            &mut crate::components::Health,
            &crate::components::Zombie,
            &Sprite,
        ),
        With<crate::components::Zombie>,
    >,
    wall_query: Query<(&Transform, &crate::components::Collider), With<crate::components::Wall>>,
    mut wallet: ResMut<crate::plugins::shop::Wallet>,
) {
    for (p_entity, p_transform, projectile) in &projectile_query {
        let mut hit_wall = false;
        // Check Wall Collision
        for (w_transform, w_collider) in &wall_query {
            let p_pos = p_transform.translation.truncate();
            let w_min = w_transform.translation.truncate() - w_collider.size / 2.0;
            let w_max = w_transform.translation.truncate() + w_collider.size / 2.0;

            if p_pos.x > w_min.x && p_pos.x < w_max.x && p_pos.y > w_min.y && p_pos.y < w_max.y {
                commands.entity(p_entity).despawn();
                hit_wall = true;
                break;
            }
        }
        if hit_wall {
            continue;
        }

        for (z_entity, z_transform, mut z_health, zombie_data, z_sprite) in &mut zombie_query {
            let distance = p_transform.translation.distance(z_transform.translation);
            let z_radius = z_sprite.custom_size.unwrap_or(Vec2::splat(20.0)).x / 2.0;
            let p_radius = 2.5; // Projectile is 5x5

            if distance < (z_radius + p_radius) {
                // Apply Damage
                z_health.current -= projectile.damage;
                commands.entity(p_entity).despawn();

                if z_health.current <= 0.0 {
                    commands.entity(z_entity).despawn();
                    wallet.money += zombie_data.money_reward;
                    info!(
                        "Zombie Killed! +${}. Current Money: {}",
                        zombie_data.money_reward, wallet.money
                    );
                }
                break;
            }
        }
    }
}

fn cleanup_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Projectile)>,
) {
    for (entity, mut projectile) in &mut query {
        projectile.lifetime.tick(time.delta());
        if projectile.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn zombie_damage_player(
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &Transform, &mut crate::components::Health),
        With<crate::plugins::player::Player>,
    >,
    zombie_query: Query<&Transform, With<crate::components::Zombie>>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
) {
    if let Ok((p_entity, p_transform, mut p_health)) = player_query.get_single_mut() {
        for z_transform in &zombie_query {
            let distance = p_transform.translation.distance(z_transform.translation);
            if distance < 32.0 {
                // Player size approx
                // Continuous damage or instant hit? Let's do continuous for now (dps)
                p_health.current -= 10.0 * time.delta_seconds(); // 10 DPS

                if p_health.current <= 0.0 {
                    // Player Death Logic
                    info!("Player Died!");
                    commands.entity(p_entity).despawn();
                    next_state.set(crate::states::GameState::GameOver);
                }
            }
        }
    }
}
