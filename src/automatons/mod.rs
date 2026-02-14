use bevy::prelude::*;

mod hellmite;
pub use hellmite::HellmitePlugin;

use crate::data::IncomeSource;

#[derive(Component)]
pub struct Automaton {
    source: IncomeSource,
    cooldown: f32,
    time_left: f32,
}
