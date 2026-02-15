use bevy::{math::ops::powf, prelude::*};
use std::collections::HashMap;

static BASE_HELLMITE_COST: u64 = 25;

#[derive(Debug, Clone, Hash, Eq, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum IncomeSource {
    Portal,
    Hellmite,
}

impl std::fmt::Display for IncomeSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IncomeSource::Portal => write!(f, "Portal"),
            IncomeSource::Hellmite => write!(f, "Hellmite"),
        }
    }
}

#[derive(Resource, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameData {
    currency: u64,
    owned_by_type: HashMap<IncomeSource, u64>,
    income_by_type: HashMap<IncomeSource, u64>,
}

impl Default for GameData {
    fn default() -> Self {
        let income_by_type: HashMap<IncomeSource, u64> = HashMap::new();
        let owned_by_type: HashMap<IncomeSource, u64> = HashMap::new();

        Self {
            currency: 0,
            owned_by_type,
            income_by_type,
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

    pub fn add_income(&mut self, source: IncomeSource, amount: u64) {
        self.currency += amount as u64;
        if let Some(income) = self.income_by_type.get_mut(&source) {
            *income += amount as u64;
        } else {
            self.income_by_type.insert(source, amount as u64);
        }
        self.save();
    }

    pub fn get_currency_by_source(&self, source: IncomeSource) -> u64 {
        self.income_by_type.get(&source).cloned().unwrap_or(0)
    }

    pub fn get_cost_to_add_source(&self, source: IncomeSource) -> u64 {
        let quantity_owned = self.get_quantity_owned_by_source(source.clone());
        f64::floor(base_cost(source) as f64 * 1.5f64.powf(quantity_owned as f64)) as u64
    }

    pub fn can_afford_source(&self, source: IncomeSource) -> bool {
        self.get_currency() >= self.get_cost_to_add_source(source)
    }

    pub fn purchase_source(&mut self, source: IncomeSource) -> bool {
        let cost = self.get_cost_to_add_source(source.clone());
        if self.can_afford_source(source.clone()) {
            self.currency -= cost;
            self.increase_quantity_owned_by_source(source);
            true
        } else {
            false
        }
    }

    pub fn increase_quantity_owned_by_source(&mut self, source: IncomeSource) {
        if let Some(quantity) = self.owned_by_type.get_mut(&source) {
            *quantity += 1;
        } else {
            self.owned_by_type.insert(source, 1);
        }
        self.save();
    }

    pub fn get_quantity_owned_by_source(&self, source: IncomeSource) -> u64 {
        self.owned_by_type.get(&source).cloned().unwrap_or(0)
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

fn base_cost(source: IncomeSource) -> u64 {
    match source {
        IncomeSource::Hellmite => BASE_HELLMITE_COST,
        _ => 0,
    }
}
