use bevy::prelude::*;
use std::collections::HashMap;

use crate::config::get_stats;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AudioSettings {
    pub volume: f32,
    pub play_pickup: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            volume: 0.25,
            play_pickup: true,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AutomatonVariant {
    Portal,
    Hellmite,
    Abyssopod,
    GapingDubine,
    GazingHoku,
    Lorgner,
    PelteLacerte,
    Struthios,
    WoolyChionoescent,
}

impl std::fmt::Display for AutomatonVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutomatonVariant::Portal => write!(f, "Portal"),
            AutomatonVariant::Hellmite => write!(f, "Hellmite"),
            AutomatonVariant::Abyssopod => write!(f, "Abyssopod"),
            AutomatonVariant::GapingDubine => write!(f, "Gaping Dubine"),
            AutomatonVariant::GazingHoku => write!(f, "Gazing Hoku"),
            AutomatonVariant::Lorgner => write!(f, "Lorgner"),
            AutomatonVariant::PelteLacerte => write!(f, "Pelte Lacerte"),
            AutomatonVariant::Struthios => write!(f, "Struthios"),
            AutomatonVariant::WoolyChionoescent => write!(f, "Wooly Chionoescent"),
        }
    }
}

#[derive(Resource, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameData {
    currency: u64,
    owned_by_type: HashMap<AutomatonVariant, u64>,
    income_by_type: HashMap<AutomatonVariant, u64>,
    #[serde(default)]
    pub audio_settings: AudioSettings,
}

impl Default for GameData {
    fn default() -> Self {
        let income_by_type: HashMap<AutomatonVariant, u64> = HashMap::new();
        let owned_by_type: HashMap<AutomatonVariant, u64> = HashMap::new();

        Self {
            currency: 0,
            owned_by_type,
            income_by_type,
            audio_settings: AudioSettings::default(),
        }
    }
}

impl GameData {
    pub fn restore() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(saved) = web_sys::window()
                .and_then(|w| w.local_storage().ok().flatten())
                .and_then(|storage| storage.get_item("game_data").ok().flatten())
            {
                serde_json::from_str(&saved).unwrap_or_default()
            } else {
                Self::default()
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(saved) = std::fs::read_to_string("save_data.json").ok() {
                serde_json::from_str(&saved).unwrap_or_default()
            } else {
                Self::default()
            }
        }
    }
}

impl GameData {
    pub fn get_currency(&self) -> u64 {
        self.currency
    }

    pub fn add_income(&mut self, source: AutomatonVariant, amount: u64) {
        self.currency += amount as u64;
        if let Some(income) = self.income_by_type.get_mut(&source) {
            *income += amount as u64;
        } else {
            self.income_by_type.insert(source, amount as u64);
        }
        self.save();
    }

    pub fn get_currency_by_source(&self, source: AutomatonVariant) -> u64 {
        self.income_by_type.get(&source).cloned().unwrap_or(0)
    }

    pub fn get_cost_to_add_source(&self, source: AutomatonVariant) -> u64 {
        let stats = get_stats(source);
        let quantity_owned = self.get_quantity_owned_by_source(source);
        f64::floor(stats.base_cost as f64 * stats.ratio.powf(quantity_owned as f64)) as u64
    }

    pub fn can_afford_source(&self, source: AutomatonVariant) -> bool {
        self.get_currency() >= self.get_cost_to_add_source(source)
    }

    pub fn purchase_source(&mut self, source: AutomatonVariant) -> bool {
        if !crate::interface::prerequisites_met(source, self) {
            return false;
        }
        let cost = self.get_cost_to_add_source(source);
        if self.can_afford_source(source) {
            self.currency -= cost;
            self.increase_quantity_owned_by_source(source);
            true
        } else {
            false
        }
    }

    pub fn increase_quantity_owned_by_source(&mut self, source: AutomatonVariant) {
        if let Some(quantity) = self.owned_by_type.get_mut(&source) {
            *quantity += 1;
        } else {
            self.owned_by_type.insert(source, 1);
        }
        self.save();
    }

    pub fn get_quantity_owned_by_source(&self, source: AutomatonVariant) -> u64 {
        self.owned_by_type.get(&source).cloned().unwrap_or(0)
    }

    pub fn update_audio_settings(&mut self, volume: f32, play_pickup: bool) {
        self.audio_settings.volume = volume;
        self.audio_settings.play_pickup = play_pickup;
        self.save();
    }

    fn save(&self) {
        let data = serde_json::to_string(self).unwrap();
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten())
            {
                let _ = storage.set_item("game_data", &data);
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = std::fs::write("save_data.json", data);
        }
    }
}
