mod pokemon;
mod attack;
mod player; 
use pokemon::Pokemon;
fn main() {
    //import pokemon module
    let mut pokemon = Pokemon::new(
        "Pikachu".to_string(),
        100,
        vec!["Electric".to_string()],
    );
    pokemon.show();
    pokemon.add_exp(130);
    pokemon.add_exp(40);
    pokemon.add_exp(200);
    pokemon.show();

    Pokemon::list();
}
