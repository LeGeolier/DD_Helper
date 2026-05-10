#[derive(Debug)]
pub struct AbilityScores {
    pub strength: AbilityScore,
    pub constitution: AbilityScore,
    pub dexterity: AbilityScore,
    pub intelligence: AbilityScore,
    pub wisdom: AbilityScore,
    pub charisma: AbilityScore,
}

#[derive(Debug)]
pub struct AbilityScore {
    pub value: u8,
}

#[derive(Debug, PartialEq)]
pub enum Ability {
    Strength,
    Constitution,
    Dexterity,
    Intelligence,
    Wisdom,
    Charisma,
}

impl AbilityScores {
    pub fn new() -> Self {
        AbilityScores {
            strength: AbilityScore::new(10),
            constitution: AbilityScore::new(10),
            dexterity: AbilityScore::new(10),
            intelligence: AbilityScore::new(10),
            wisdom: AbilityScore::new(10),
            charisma: AbilityScore::new(10),
        }
    }

    pub fn iter(&self) -> [(&str, &AbilityScore); 6] {
        [
            ("STR", &self.strength),
            ("DEX", &self.dexterity),
            ("CONST", &self.constitution),
            ("INT", &self.intelligence),
            ("WIS", &self.wisdom),
            ("CHAR", &self.charisma),
        ]
    }
}

impl AbilityScore {
    pub fn new(value: u8) -> Self {
        AbilityScore { value }
    }

    pub fn modifier(&self) -> i8 {
        let value_signed: i8 = self.value as i8;
        (value_signed - 10).div_euclid(2)
    }
}

impl Ability {
    pub fn ability_modifier(&self, scores: &AbilityScores) -> i8 {
        match self {
            Ability::Charisma => scores.charisma.modifier(),
            Ability::Constitution => scores.constitution.modifier(),
            Ability::Dexterity => scores.dexterity.modifier(),
            Ability::Intelligence => scores.intelligence.modifier(),
            Ability::Strength => scores.strength.modifier(),
            Ability::Wisdom => scores.wisdom.modifier(),
        }
    }
}
