use crate::AbilityScores;
use crate::Class;
use crate::Race;
use crate::ability::Ability;
use crate::skills::Skills;

pub struct Character {
    name: String,
    surname: String,
    player_name: String,
    ability_scores: AbilityScores,
    max_hp: i32,
    current_hp: i32,
    race: Race,
    class: Class,
    level: u8,
    proeficient_skills: Vec<Skills>,
    saving_throw_proficiencies: Vec<Ability>,
    armor_class: i32,
}

impl Character {
    pub fn new(
        name: String,
        surname: String,
        player_name: String,
        max_hp: i32,
        race: Race,
        class: Class,
        level: u8,
    ) -> Self {
        let ability_scores: AbilityScores = AbilityScores::new();
        let init_skills: Vec<Skills> = Vec::new();
        let init_saving_throw_skills: Vec<Ability> = Vec::new();
        Character {
            name,
            surname,
            player_name,
            ability_scores,
            max_hp,
            current_hp: max_hp,
            race,
            class,
            level,
            proeficient_skills: init_skills,
            saving_throw_proficiencies: init_saving_throw_skills,
            armor_class: 10,
        }
    }

    pub fn proficiency(&self) -> u8 {
        (self.level - 1) / 4 + 2
    }

    pub fn get_skill_modifier(&self, target_skill: &Skills) -> i8 {
        let base = target_skill.skill_modifier(self.ability_scores());
        if self.proeficient_skills().contains(target_skill) {
            base + self.proficiency() as i8
        } else {
            base
        }
    }

    pub fn get_save_modifier(&self, target_save_throw: &Ability) -> i8 {
        let base = target_save_throw.ability_modifier(self.ability_scores());
        if self.proeficient_saving_throw().contains(target_save_throw) {
            base + self.proficiency() as i8
        } else {
            base
        }
    }

    pub fn gain_proficiency_in_skills(&mut self, new_skill: Skills) {
        self.proeficient_skills.push(new_skill);
    }

    pub fn gain_proficiency_in_ability(&mut self, new_ability: Ability) {
        self.saving_throw_proficiencies.push(new_ability);
    }

    pub fn gain_saving_throw_proficiency(&mut self, new_proficiency: Ability) {
        self.saving_throw_proficiencies.push(new_proficiency);
    }

    pub fn apply_dmg(&mut self, damage: i32) {
        self.current_hp -= damage;
    }

    pub fn heal_hp(&mut self, heal: i32) {
        self.current_hp += heal;
        if self.current_hp > self.max_hp {
            self.current_hp = self.max_hp;
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn surname(&self) -> &str {
        &self.surname
    }
    pub fn player_name(&self) -> &str {
        &self.player_name
    }
    pub fn max_hp(&self) -> i32 {
        self.max_hp
    }
    pub fn current_hp(&self) -> i32 {
        self.current_hp
    }
    pub fn ability_scores(&self) -> &AbilityScores {
        &self.ability_scores
    }
    pub fn race(&self) -> &Race {
        &self.race
    }
    pub fn class(&self) -> &Class {
        &self.class
    }
    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn proeficient_skills(&self) -> &Vec<Skills> {
        &self.proeficient_skills
    }

    pub fn proeficient_saving_throw(&self) -> &Vec<Ability> {
        &self.saving_throw_proficiencies
    }

    pub fn display_sheet(&self) {
        println!("Name : {}", self.name());
        println!("Surname : {}", self.surname());
        println!("Player name : {}", self.player_name());
        println!("Race : {}", self.race());
        println!("Class : {}", self.class());
        println!("HP : {}/{}", self.current_hp(), self.max_hp());
        println!("Proficiency : +{}", self.proficiency());

        for ability in self.ability_scores().iter() {
            println!(
                "{}: {} ({})",
                ability.0,
                ability.1.value,
                ability.1.modifier()
            );
        }
    }
}
