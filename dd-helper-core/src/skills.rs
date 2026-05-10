use crate::AbilityScores;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Skills {
    Athletics,
    Acrobatics,
    SleightofHand,
    Stealth,
    Arcana,
    History,
    Investigation,
    Nature,
    Religion,
    AnimalHandling,
    Insight,
    Medicine,
    Perception,
    Survival,
    Deception,
    Intimidation,
    Performance,
    Persuasion,
}

impl fmt::Display for Skills {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Skills::SleightofHand => write!(f, "Sleight of Hand"),
            Skills::AnimalHandling => write!(f, "Animal Handling"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Skills {
    pub fn skill_modifier(&self, scores: &AbilityScores) -> i8 {
        match self {
            Skills::Athletics => scores.strength.modifier(),
            Skills::Acrobatics => scores.dexterity.modifier(),
            Skills::SleightofHand => scores.dexterity.modifier(),
            Skills::Stealth => scores.dexterity.modifier(),
            Skills::Arcana => scores.intelligence.modifier(),
            Skills::History => scores.intelligence.modifier(),
            Skills::Investigation => scores.intelligence.modifier(),
            Skills::Nature => scores.intelligence.modifier(),
            Skills::Religion => scores.intelligence.modifier(),
            Skills::AnimalHandling => scores.wisdom.modifier(),
            Skills::Insight => scores.wisdom.modifier(),
            Skills::Medicine => scores.wisdom.modifier(),
            Skills::Perception => scores.wisdom.modifier(),
            Skills::Survival => scores.wisdom.modifier(),
            Skills::Deception => scores.charisma.modifier(),
            Skills::Intimidation => scores.charisma.modifier(),
            Skills::Performance => scores.charisma.modifier(),
            Skills::Persuasion => scores.charisma.modifier(),
        }
    }
}
