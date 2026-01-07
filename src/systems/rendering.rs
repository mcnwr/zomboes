use bevy::prelude::*;

pub fn y_sort(mut query: Query<&mut Transform, (Changed<Transform>, With<Sprite>)>) {
    for mut transform in &mut query {
        // -Y because higher Y is "further back" in 2D top-down.
        // We add a base Z of 0.5 to keep it above the floor (if floor is 0).
        // Scaling by 0.001 keeps it strictly ordered but close to the base order.
        let y = transform.translation.y;
        transform.translation.z = 0.5 - y * 0.001; 
    }
}
