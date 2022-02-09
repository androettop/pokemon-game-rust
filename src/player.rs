use crate::pokemon::Pokemon;

const SEPARATOR: &str = "==========================================";

enum PlayerType {
    Human,
    Computer,
}

pub struct Player {
    name: String,
    player_type: PlayerType,
    pokedex: Vec<Pokemon>,
    current_pokemon: usize,
    current_attack: usize,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            player_type: PlayerType::Human,
            pokedex: vec![],
            current_pokemon: 0,
            current_attack: 0,
        }
    }

    pub fn init_cpu(&mut self) {
        self.player_type = PlayerType::Computer;
    }

    pub fn init_human(&mut self) {
        self.player_type = PlayerType::Human;
    }

    pub fn init(&mut self) {
        match self.player_type {
            PlayerType::Human => self.init_human(),
            PlayerType::Computer => self.init_cpu(),
        }
    }
}
