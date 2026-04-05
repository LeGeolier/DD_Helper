use dd_helper_core::AbilityScores;

fn main() {
    let mut a = AbilityScores::new();
    a.strength.value = 16;
    println!("{:?}", a);

    for ability in a.iter() {
        println!("{}: {} ({})",ability.0,ability.1.value,ability.1.modifier());
    }
}
