use std::fmt;

#[derive(Debug)]
pub enum Race {
    Aarakocra,
    Aasimar,
    Bugbear,
    Centaur,
    DeepGnome,
    Deva,
    Dragonborn,
    Drow,
    Duergar,
    Dwarf,
    Eladrin,
    Elf,
    Fairies,
    Firbolg,
    Genasi,
    Githyanki,
    Githzerai,
    Gnome,
    Goblin,
    Goliath,
    Grung,
    HalfElf,
    HalfOrc,
    Halfling,
    Harengon,
    Hobgoblin,
    Human,
    Kalashtar,
    Kenku,
    Kobold,
    Leonin,
    Lizardfolk,
    Locathah,
    Loxodon,
    Minotaur,
    Ork,
    Satyr,
    ShadarKai,
    Shardmind,
    Shifter,
    SimicHybrid,
    Tabaxi,
    Tiefling,
    Tortle,
    Triton,
    Vedalken,
    Verdan,
    Warforged,
    Wilden,
    YuanTi,
}
#[derive(Debug)]
pub enum Class {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
    Artificer,
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Race::HalfElf => write!(f, "Half-Elf"),
            Race::HalfOrc => write!(f, "Half-Orc"),
            Race::DeepGnome => write!(f, "Deep Gnome"),
            Race::SimicHybrid => write!(f, "Simic Hybrid"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "{:?}", self),
        }
    }
}
