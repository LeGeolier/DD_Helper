#[derive(Debug)]
pub struct AbilityScores {
    pub strength: Ability,
    pub constitution: Ability,
    pub dexterity: Ability,
    pub intelligence: Ability,
    pub wisdom: Ability,
    pub charisma: Ability,
}

#[derive(Debug)]
pub struct Ability {
    pub value: u8,
}

impl AbilityScores {
    pub fn new() -> Self {
        AbilityScores {
            strength: Ability::new(10),
            constitution: Ability::new(10),
            dexterity: Ability::new(10),
            intelligence: Ability::new(10),
            wisdom: Ability::new(10),
            charisma: Ability::new(10),
        }
    }

    pub fn iter(&self) -> [(&str, &Ability); 6] {
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

impl Ability {
    pub fn new(value: u8) -> Self {
        Ability { value }
    }

    pub fn modifier(&self) -> i8 {
        let value_signed: i8 = self.value as i8;
        (value_signed - 10).div_euclid(2)
    }
}
