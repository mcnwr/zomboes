use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Wallet {
    pub money: u32,
}

#[derive(Resource)]
pub struct WeaponUpgradeCost {
    pub fire_rate_cost: u32,
}

impl Default for WeaponUpgradeCost {
    fn default() -> Self {
        Self { fire_rate_cost: 10 }
    }
}

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Wallet>()
           .init_resource::<WeaponUpgradeCost>()
           .add_systems(Update, shop_input.run_if(in_state(crate::states::GameState::Playing)));
    }
}

fn shop_input(
    _commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut wallet: ResMut<Wallet>,
    mut cost: ResMut<WeaponUpgradeCost>,
    mut player_query: Query<&mut crate::plugins::player::WeaponStats>, // We need to add WeaponStats to player
) {
    if input.just_pressed(KeyCode::KeyB) {
        if wallet.money >= cost.fire_rate_cost {
             // Upgrade Logic
             if let Ok(mut stats) = player_query.get_single_mut() {
                 wallet.money -= cost.fire_rate_cost;
                 stats.fire_rate *= 0.8; // Decrease cooldown
                 // Apply new duration immediately
                 let new_rate = stats.fire_rate;
                 stats.timer.set_duration(std::time::Duration::from_secs_f32(new_rate));
                 
                 cost.fire_rate_cost += 50; // Increase price
                 info!("Weapon Upgraded! New Fire Rate: {:.2}", stats.fire_rate);
             }
        } else {
            info!("Not enough money! Need ${}", cost.fire_rate_cost);
        }
    }
}
