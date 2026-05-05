use bevy::prelude::*;
use std::collections::HashMap;

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

#[derive(Debug, Clone, Copy)]
pub struct AutomatonStats {
    pub distance_from_origin: f32,
    pub cooldown: f32,
    pub currency_per_tick: u64,
    pub scale: f32,
    pub base_cost: u64,
    pub ratio: f64,
    pub rotation: f32,
    pub level_up_cost: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    Portal,
    Automaton { asset_name: &'static str },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnlockRequirement {
    None,
    FirstPurchaseCost,
    PreviousAutomaton {
        variant: AutomatonVariant,
        quantity: u64,
    },
}

impl UnlockRequirement {
    fn is_met(self, source: AutomatonVariant, game_data: &GameData) -> bool {
        match self {
            UnlockRequirement::None => true,
            UnlockRequirement::FirstPurchaseCost => {
                game_data.get_quantity_owned_by_source(source) > 0
                    || game_data.can_afford_source(source)
            }
            UnlockRequirement::PreviousAutomaton { variant, quantity } => {
                game_data.get_quantity_owned_by_source(variant) >= quantity
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SourceDefinition {
    pub variant: AutomatonVariant,
    pub display_name: &'static str,
    pub plural_display_name: &'static str,
    pub kind: SourceKind,
    pub stats: AutomatonStats,
    pub unlock_requirement: UnlockRequirement,
}

impl SourceDefinition {
    pub fn is_automaton(&self) -> bool {
        matches!(self.kind, SourceKind::Automaton { .. })
    }

    pub fn asset_name(&self) -> Option<&'static str> {
        match self.kind {
            SourceKind::Automaton { asset_name } => Some(asset_name),
            SourceKind::Portal => None,
        }
    }

    pub fn model_path(&self) -> Option<String> {
        self.asset_name()
            .map(|asset_name| format!("models/{asset_name}.glb"))
    }

    pub fn ring_name(&self) -> Option<String> {
        self.asset_name()
            .map(|asset_name| format!("{asset_name}_ring"))
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

impl AutomatonVariant {
    pub fn definition(self) -> &'static SourceDefinition {
        source_definition(self)
    }

    pub fn stats(self) -> &'static AutomatonStats {
        &self.definition().stats
    }

    pub fn is_automaton(self) -> bool {
        self.definition().is_automaton()
    }

    pub fn display_name(self) -> &'static str {
        self.definition().display_name
    }

    pub fn label_for_quantity(self, quantity: u64) -> &'static str {
        if quantity == 1 {
            self.definition().display_name
        } else {
            self.definition().plural_display_name
        }
    }
}

impl std::fmt::Display for AutomatonVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

pub const SOURCE_DEFINITIONS: [SourceDefinition; 9] = [
    SourceDefinition {
        variant: AutomatonVariant::Hellmite,
        display_name: "Hellmite",
        plural_display_name: "Hellmites",
        kind: SourceKind::Automaton {
            asset_name: "hellmite",
        },
        stats: AutomatonStats {
            distance_from_origin: 2.5,
            cooldown: 2.5,
            currency_per_tick: 1,
            scale: 0.25,
            base_cost: 25,
            ratio: 1.05,
            rotation: 0.05,
            level_up_cost: 50,
        },
        unlock_requirement: UnlockRequirement::FirstPurchaseCost,
    },
    SourceDefinition {
        variant: AutomatonVariant::Abyssopod,
        display_name: "Abyssopod",
        plural_display_name: "Abyssopods",
        kind: SourceKind::Automaton {
            asset_name: "abyssopod",
        },
        stats: AutomatonStats {
            distance_from_origin: 3.5,
            cooldown: 7.5,
            currency_per_tick: 20,
            scale: 0.35,
            base_cost: 100,
            ratio: 1.1,
            rotation: -0.05,
            level_up_cost: 50,
        },
        unlock_requirement: UnlockRequirement::PreviousAutomaton {
            variant: AutomatonVariant::Hellmite,
            quantity: 20,
        },
    },
    SourceDefinition {
        variant: AutomatonVariant::GapingDubine,
        display_name: "Gaping Dubine",
        plural_display_name: "Gaping Dubines",
        kind: SourceKind::Automaton {
            asset_name: "gaping_dubine",
        },
        stats: AutomatonStats {
            distance_from_origin: 5.0,
            cooldown: 15.0,
            currency_per_tick: 45,
            scale: 0.5,
            base_cost: 500,
            ratio: 1.25,
            rotation: 0.05,
            level_up_cost: 50,
        },
        unlock_requirement: UnlockRequirement::PreviousAutomaton {
            variant: AutomatonVariant::Abyssopod,
            quantity: 15,
        },
    },
    SourceDefinition {
        variant: AutomatonVariant::GazingHoku,
        display_name: "Gazing Hoku",
        plural_display_name: "Gazing Hokus",
        kind: SourceKind::Automaton {
            asset_name: "gazing_hoku",
        },
        stats: AutomatonStats {
            distance_from_origin: 7.0,
            cooldown: 30.0,
            currency_per_tick: 120,
            scale: 0.6,
            base_cost: 2500,
            ratio: 1.45,
            rotation: -0.05,
            level_up_cost: 50,
        },
        unlock_requirement: UnlockRequirement::PreviousAutomaton {
            variant: AutomatonVariant::GapingDubine,
            quantity: 10,
        },
    },
    SourceDefinition {
        variant: AutomatonVariant::Lorgner,
        display_name: "Lorgner",
        plural_display_name: "Lorgners",
        kind: SourceKind::Automaton {
            asset_name: "lorgner",
        },
        stats: AutomatonStats {
            distance_from_origin: 10.0,
            cooldown: 50.0,
            currency_per_tick: 625,
            scale: 0.75,
            base_cost: 12500,
            ratio: 1.6,
            rotation: 0.05,
            level_up_cost: 50,
        },
        unlock_requirement: UnlockRequirement::PreviousAutomaton {
            variant: AutomatonVariant::GazingHoku,
            quantity: 8,
        },
    },
    SourceDefinition {
        variant: AutomatonVariant::PelteLacerte,
        display_name: "Pelte Lacerte",
        plural_display_name: "Pelte Lacertes",
        kind: SourceKind::Automaton {
            asset_name: "pelte_lacerte",
        },
        stats: AutomatonStats {
            distance_from_origin: 13.0,
            cooldown: 60.0,
            currency_per_tick: 1500,
            scale: 0.8,
            base_cost: 62500,
            ratio: 1.75,
            rotation: -0.05,
            level_up_cost: 50,
        },
        unlock_requirement: UnlockRequirement::PreviousAutomaton {
            variant: AutomatonVariant::Lorgner,
            quantity: 6,
        },
    },
    SourceDefinition {
        variant: AutomatonVariant::Struthios,
        display_name: "Struthios",
        plural_display_name: "Struthios",
        kind: SourceKind::Automaton {
            asset_name: "struthios",
        },
        stats: AutomatonStats {
            distance_from_origin: 16.0,
            cooldown: 90.0,
            currency_per_tick: 5000,
            scale: 0.9,
            base_cost: 62500,
            ratio: 1.8,
            rotation: 0.05,
            level_up_cost: 50,
        },
        unlock_requirement: UnlockRequirement::PreviousAutomaton {
            variant: AutomatonVariant::PelteLacerte,
            quantity: 5,
        },
    },
    SourceDefinition {
        variant: AutomatonVariant::WoolyChionoescent,
        display_name: "Wooly Chionoescent",
        plural_display_name: "Wooly Chionoescents",
        kind: SourceKind::Automaton {
            asset_name: "wooly_chionoescent",
        },
        stats: AutomatonStats {
            distance_from_origin: 20.0,
            cooldown: 120.0,
            currency_per_tick: 150000,
            scale: 1.0,
            base_cost: 1562500,
            ratio: 2.0,
            rotation: -0.05,
            level_up_cost: 50,
        },
        unlock_requirement: UnlockRequirement::PreviousAutomaton {
            variant: AutomatonVariant::Struthios,
            quantity: 2,
        },
    },
    SourceDefinition {
        variant: AutomatonVariant::Portal,
        display_name: "Portal",
        plural_display_name: "Portals",
        kind: SourceKind::Portal,
        stats: AutomatonStats {
            distance_from_origin: 0.0,
            cooldown: 1.0,
            currency_per_tick: 0,
            scale: 0.0,
            base_cost: 0,
            ratio: 1.0,
            rotation: 0.0,
            level_up_cost: 50,
        },
        unlock_requirement: UnlockRequirement::None,
    },
];

pub fn source_definition(variant: AutomatonVariant) -> &'static SourceDefinition {
    SOURCE_DEFINITIONS
        .iter()
        .find(|definition| definition.variant == variant)
        .expect("missing source definition")
}

pub fn automaton_definitions() -> impl Iterator<Item = &'static SourceDefinition> {
    SOURCE_DEFINITIONS
        .iter()
        .filter(|definition| definition.is_automaton())
}

const LEVEL_MULTIPLIER_BASE: f64 = 1.25;

#[derive(Resource, Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GameData {
    currency: u64,
    owned_by_type: HashMap<AutomatonVariant, u64>,
    income_by_type: HashMap<AutomatonVariant, u64>,
    #[serde(default)]
    levels_by_type: HashMap<AutomatonVariant, u32>,
    #[serde(default)]
    pub audio_settings: AudioSettings,
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
            if let Ok(saved) = std::fs::read_to_string("save_data.json") {
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
        let scaled = self.scaled_amount(source, amount);
        self.currency += scaled;
        *self.income_by_type.entry(source).or_insert(0) += scaled;
        self.save();
    }

    fn scaled_amount(&self, source: AutomatonVariant, amount: u64) -> u64 {
        let scaled = amount as f64 * self.level_multiplier(source);
        scaled.floor().max(0.0) as u64
    }

    pub fn get_level(&self, source: AutomatonVariant) -> u32 {
        self.levels_by_type.get(&source).copied().unwrap_or(0)
    }

    pub fn level_multiplier(&self, source: AutomatonVariant) -> f64 {
        LEVEL_MULTIPLIER_BASE.powi(self.get_level(source) as i32)
    }

    pub fn cost_to_level_up(&self, source: AutomatonVariant) -> u64 {
        source.stats().level_up_cost
    }

    pub fn can_level_up(&self, source: AutomatonVariant) -> bool {
        source.is_automaton()
            && self.get_quantity_owned_by_source(source) >= self.cost_to_level_up(source)
    }

    pub fn level_up(&mut self, source: AutomatonVariant) -> bool {
        if !self.can_level_up(source) {
            return false;
        }
        let cost = self.cost_to_level_up(source);
        let owned = self.owned_by_type.entry(source).or_insert(0);
        *owned -= cost;
        *self.levels_by_type.entry(source).or_insert(0) += 1;
        self.save();
        true
    }

    pub fn get_currency_by_source(&self, source: AutomatonVariant) -> u64 {
        self.income_by_type.get(&source).cloned().unwrap_or(0)
    }

    pub fn get_cost_to_add_source(&self, source: AutomatonVariant) -> u64 {
        let stats = source.stats();
        let quantity_owned = self.get_quantity_owned_by_source(source);
        f64::floor(stats.base_cost as f64 * stats.ratio.powf(quantity_owned as f64)) as u64
    }

    pub fn can_afford_source(&self, source: AutomatonVariant) -> bool {
        self.get_currency() >= self.get_cost_to_add_source(source)
    }

    pub fn purchase_source(&mut self, source: AutomatonVariant) -> bool {
        if !source.is_automaton() || !self.prerequisites_met(source) {
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
        *self.owned_by_type.entry(source).or_insert(0) += 1;
        self.save();
    }

    pub fn get_quantity_owned_by_source(&self, source: AutomatonVariant) -> u64 {
        self.owned_by_type.get(&source).cloned().unwrap_or(0)
    }

    pub fn rate_per_second_by_source(&self, source: AutomatonVariant) -> f64 {
        let stats = source.stats();
        let raw = stats.currency_per_tick as f64 / stats.cooldown as f64
            * self.get_quantity_owned_by_source(source) as f64;
        raw * self.level_multiplier(source)
    }

    pub fn prerequisites_met(&self, source: AutomatonVariant) -> bool {
        source.is_automaton() && source.definition().unlock_requirement.is_met(source, self)
    }

    pub fn unmet_unlock_requirement(&self, source: AutomatonVariant) -> Option<UnlockRequirement> {
        let requirement = source.definition().unlock_requirement;
        if requirement.is_met(source, self) {
            None
        } else {
            Some(requirement)
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_definitions_have_unique_variants() {
        for definition in SOURCE_DEFINITIONS {
            let count = SOURCE_DEFINITIONS
                .iter()
                .filter(|other| other.variant == definition.variant)
                .count();
            assert_eq!(
                count, 1,
                "{:?} is defined more than once",
                definition.variant
            );
        }
    }

    #[test]
    fn automaton_asset_paths_are_derived_from_asset_name() {
        let hellmite = AutomatonVariant::Hellmite.definition();

        assert_eq!(hellmite.asset_name(), Some("hellmite"));
        assert_eq!(
            hellmite.model_path().as_deref(),
            Some("models/hellmite.glb")
        );
        assert_eq!(hellmite.ring_name().as_deref(), Some("hellmite_ring"));
    }

    #[test]
    fn unlock_requirements_are_checked_from_source_definitions() {
        let mut game_data = GameData::default();

        assert!(!game_data.prerequisites_met(AutomatonVariant::Hellmite));
        assert!(!game_data.prerequisites_met(AutomatonVariant::Abyssopod));

        game_data.currency = 25;
        assert!(game_data.prerequisites_met(AutomatonVariant::Hellmite));

        game_data
            .owned_by_type
            .insert(AutomatonVariant::Hellmite, 20);

        assert!(game_data.prerequisites_met(AutomatonVariant::Abyssopod));
    }
}
