use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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

#[derive(Resource, Debug, Clone)]
pub struct GameData {
    currency: u64,
    income_by_type: HashMap<IncomeSource, u64>,
}

impl Default for GameData {
    fn default() -> Self {
        let mut income_by_type: HashMap<IncomeSource, u64> = HashMap::new();
        income_by_type.insert(IncomeSource::Portal, 0);

        Self {
            currency: 0,
            income_by_type,
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
    }

    pub fn get_currency_by_source(&self, source: IncomeSource) -> u64 {
        self.income_by_type.get(&source).cloned().unwrap_or(0)
    }
}
