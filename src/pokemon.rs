use rand;
use crate::attack::Attack;
use std::fs;
use serde_json::{Result, Value};


const EXP_LEVEL: [u32; 3] = [150, 200, 250];
const SEPARATOR: &str = "==========================================";

pub struct Pokemon {
    id: u32,
    name: String,
    health: u32,
    base_health: u32,
    level: u32,
    types: Vec<String>,
    exp: u32,
    attacks: Vec<Attack>,
}

impl Pokemon {
    pub fn list<'a>() -> Result<&'a Vec<Value>> {
        let data = fs::read_to_string("./data.json")
        .expect("Something went wrong reading the file");

        let v: Value = serde_json::from_str(&data)?;
        Ok(v["pokemon_list"].as_array().unwrap())
   }

    pub fn new(name: String, base_health: u32, types: Vec<String>) -> Pokemon {
        //generate random id
        let id = rand::random();
        Pokemon {
            id,
            name,
            health: base_health,
            base_health,
            level: 1,
            types,
            exp: 0,
            attacks: vec![],
        }
    }

    pub fn show(&self) {
        println!("{}", SEPARATOR);
        println!(
            "{name} (type/s {types})",
            name = self.name,
            types = self.types.join(", "),
        );
        println!(
            "Health: {health}/{base_health} - Level: {level}, Exp: {exp}",
            health = self.health,
            base_health = self.base_health,
            level = self.level,
            exp = self.exp
        );
        println!("{}", SEPARATOR);
    }

    pub fn restore(&mut self) {
        self.health = self.base_health;
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.base_health += 10;
        self.restore();
        println!("{}", SEPARATOR);
        println!("{name} has leveled up to {level}!", name = self.name, level = self.level);
        println!("{}", SEPARATOR);
    }

    pub fn add_exp(&mut self, exp: u32) {
        if self.health > 0 {
            let mut exp_needed = 0;
            if self.level as usize > EXP_LEVEL.len() {
                exp_needed = EXP_LEVEL[EXP_LEVEL.len() - 1];
            } else {
                exp_needed = EXP_LEVEL[self.level as usize - 1];
            }
            self.exp += exp;

            println!("{}", SEPARATOR);
            println!("{name} has gained {exp} exp!", name = self.name, exp = exp);
            println!("{}", SEPARATOR);

            if self.exp >= exp_needed {
                self.level_up();
                self.exp -= exp_needed;
            }
        }
    }
    
    
}
