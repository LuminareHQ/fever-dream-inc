use bevy::prelude::*;

mod hellmite;
mod abyssopod;
mod gaping_dubine;
mod gazing_hoku;
mod lorgner;
mod pelte_lacerte;
mod struthios;
mod wooly_chionoescent;
pub use hellmite::HellmitePlugin;
pub use abyssopod::AbyssopodPlugin;
pub use gaping_dubine::GapingDubinePlugin;
pub use gazing_hoku::GazingHokuPlugin;
pub use lorgner::LorgnerPlugin;
pub use pelte_lacerte::PelteLacertePlugin;
pub use struthios::StruthiosPlugin;
pub use wooly_chionoescent::WoolyChionoescentPlugin;

use crate::data::IncomeSource;

#[derive(Component)]
pub struct Automaton {
    source: IncomeSource,
    currency_per_tick: u64,
    cooldown: f32,
    time_left: f32,
}

pub fn update_automatons(
    mut automatons: Query<(&mut Automaton, &Transform), Without<AutomatonOrb>>,
    time: Res<Time>,
    mut data: ResMut<crate::data::GameData>,
    mut orbs: Query<(&mut Transform, &mut AutomatonOrb, &mut Visibility)>,
) {
    for (mut automaton, entity_transform) in automatons.iter_mut() {
        if automaton.time_left >= 0.0 {
            automaton.time_left -= time.delta_secs();
        } else {
            data.add_income(automaton.source.clone(), automaton.currency_per_tick);
            automaton.time_left = automaton.cooldown;
            for (mut orb_transform, mut orb, mut vis) in orbs.iter_mut() {
                if orb_transform.translation.distance(Vec3::ZERO) < 0.1 {
                    orb.start = entity_transform.translation;
                    orb_transform.translation = orb.start;
                    orb.progress = 0.0;
                    break;
                }
            }
        }
    }

    for (mut transform, mut orb, mut vis) in orbs.iter_mut() {
        if transform.translation.distance(Vec3::ZERO) > 0.1 {
            *vis = Visibility::Visible;
            orb.update(time.delta_secs());
            let t = orb.progress / 0.5;
            transform.translation = orb.start.lerp(Vec3::ZERO, t);
        } else {
            *vis = Visibility::Hidden;
        }
    }
}

#[derive(Component)]
pub struct AutomatonOrb {
    start: Vec3,
    progress: f32,
}

impl Default for AutomatonOrb {
    fn default() -> Self {
        Self {
            start: Vec3::ZERO,
            progress: 1.0,
        }
    }
}

impl AutomatonOrb {
    pub fn new(start: Vec3) -> Self {
        Self {
            start,
            progress: 0.0,
        }
    }

    pub fn update(&mut self, time: f32) {
        self.progress += time;
    }
}
