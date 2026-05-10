use dd_helper_core::Character;
use dd_helper_core::Class;
use dd_helper_core::Race;

fn main() {
    let mut me = Character::new(
        String::from("Gaston"),
        String::from("LeVostre"),
        String::from("Jean Roelens"),
        36,
        Race::SimicHybrid,
        Class::Fighter,
        1,
    );
    me.display_sheet();
    me.apply_dmg(13);
    me.display_sheet();
    me.heal_hp(50);
    me.display_sheet();
}
