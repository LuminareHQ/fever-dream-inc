use crate::data::AutomatonVariant;

pub struct AutomatonStats {
    // The distance from the origin that the automaton is placed at, in world units
    pub distance_from_origin: f32,
    // The cooldown of the automaton, in seconds, between each tick activation
    pub cooldown: f32,
    // The amount of currency this automaton generates per tick activation
    pub currency_per_tick: u64,
    // The scale of the automaton, used for both the model and the info ring
    pub scale: f32,
    // The amount of current required to unlock the first automaton of this type
    pub base_cost: u64,
    // The ratio used to calculate the cost of each subsequent automaton of this type, using the formula: base_cost * ratio^quantity_owned
    pub ratio: f64,
    // For the Hellmites, the amount of required currency, for the rest, the amount of the previous automaton required to unlock this one
    pub required_previous: u64,
    // Rotation Direction & Speed
    pub rotation: f32,
}

pub const fn get_stats(variant: AutomatonVariant) -> &'static AutomatonStats {
    match variant {
        AutomatonVariant::Hellmite => &AUTOMATON_STATS[0].1,
        AutomatonVariant::Abyssopod => &AUTOMATON_STATS[1].1,
        AutomatonVariant::GapingDubine => &AUTOMATON_STATS[2].1,
        AutomatonVariant::GazingHoku => &AUTOMATON_STATS[3].1,
        AutomatonVariant::Lorgner => &AUTOMATON_STATS[4].1,
        AutomatonVariant::PelteLacerte => &AUTOMATON_STATS[5].1,
        AutomatonVariant::Struthios => &AUTOMATON_STATS[6].1,
        AutomatonVariant::WoolyChionoescent => &AUTOMATON_STATS[7].1,
        _ => unreachable!(),
    }
}

pub const AUTOMATON_STATS: [(AutomatonVariant, AutomatonStats); 8] = [
    (
        AutomatonVariant::Hellmite,
        AutomatonStats {
            distance_from_origin: 2.5,
            cooldown: 5.0,
            currency_per_tick: 1,
            scale: 0.25,
            base_cost: 25,
            ratio: 1.25,
            required_previous: 25,
            rotation: 0.05,
        },
    ),
    (
        AutomatonVariant::Abyssopod,
        AutomatonStats {
            distance_from_origin: 3.5,
            cooldown: 15.0,
            currency_per_tick: 10,
            scale: 0.35,
            base_cost: 100,
            ratio: 1.35,
            required_previous: 5,
            rotation: -0.05,
        },
    ),
    (
        AutomatonVariant::GapingDubine,
        AutomatonStats {
            distance_from_origin: 5.0,
            cooldown: 30.0,
            currency_per_tick: 25,
            scale: 0.5,
            base_cost: 500,
            ratio: 1.55,
            required_previous: 10,
            rotation: 0.05,
        },
    ),
    (
        AutomatonVariant::GazingHoku,
        AutomatonStats {
            distance_from_origin: 7.0,
            cooldown: 40.0,
            currency_per_tick: 75,
            scale: 0.6,
            base_cost: 2500,
            ratio: 1.7,
            required_previous: 10,
            rotation: -0.05,
        },
    ),
    (
        AutomatonVariant::Lorgner,
        AutomatonStats {
            distance_from_origin: 10.0,
            cooldown: 50.0,
            currency_per_tick: 625,
            scale: 0.75,
            base_cost: 12500,
            ratio: 1.95,
            required_previous: 10,
            rotation: 0.05,
        },
    ),
    (
        AutomatonVariant::PelteLacerte,
        AutomatonStats {
            distance_from_origin: 13.0,
            cooldown: 60.0,
            currency_per_tick: 2000,
            scale: 0.8,
            base_cost: 62500,
            ratio: 2.1,
            required_previous: 10,
            rotation: -0.05,
        },
    ),
    (
        AutomatonVariant::Struthios,
        AutomatonStats {
            distance_from_origin: 16.0,
            cooldown: 90.0,
            currency_per_tick: 6500,
            scale: 0.9,
            base_cost: 312500,
            ratio: 2.5,
            required_previous: 10,
            rotation: 0.05,
        },
    ),
    (
        AutomatonVariant::WoolyChionoescent,
        AutomatonStats {
            distance_from_origin: 20.0,
            cooldown: 120.0,
            currency_per_tick: 32000,
            scale: 1.0,
            base_cost: 1562500,
            ratio: 2.75,
            required_previous: 10,
            rotation: -0.05,
        },
    ),
];
