use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_level);
    }
}

fn spawn_level(mut _commands: Commands) {
    // Wall Material (Grey)
    let _wall_color = Color::srgb(0.5, 0.5, 0.5);
    let _wall_size = Vec2::new(40.0, 40.0);

    // Simple Housing Complex Layout
    // A few "Houses" (Rectangles of walls)

    // House 1 (Top Left)
    // spawn_wall_rect(&mut commands, Vec2::new(-200.0, 200.0), 100.0, 80.0, wall_color);

    // House 2 (Bottom Right)
    // spawn_wall_rect(&mut commands, Vec2::new(200.0, -150.0), 120.0, 100.0, wall_color);

    // Some random debris/walls
    // spawn_wall(&mut commands, Vec2::new(0.0, 50.0), wall_size, wall_color);
    // spawn_wall(&mut commands, Vec2::new(50.0, -50.0), wall_size, wall_color);
}
