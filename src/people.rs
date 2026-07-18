use std::fmt;
use rand;

struct Person {
    name: String,
    age: i32,
    gender: Gender,
    alignment: Alignment,
    damage: i32,
    health: i32,
    dead: bool,
}
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is {} years old and {}",
    self.name, self.age, self.gender)
    }
}
impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is {} years old and {}",
    self.name, self.age, self.gender)
    }
}

impl Person {
    fn attack(&mut self, attacker: &Person) {
        self.health -= attacker.damage;
        println!(
            "{} attacks {} and deals {} damage.",
            attacker.name, self.name, attacker.damage
        );
        if self.health <= 0 {
            println!("{} is dead.", self.name);
            self.dead = true;
        }
    }
}
// TODO: ADD GENERIC GENDER TYPE
enum Gender {
    Male,
    Female, 
    Nonbinary,
    Agender,
}
impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
            Gender::Nonbinary => write!(f, "Nonbinary"),
            Gender::Agender => write!(f, "Agender")
        }
    }
}
enum Alignment {
    Good,
    Evil,
    Neutral,
}

pub fn people() {

    let list = [
        Person {
            name: String::from("Sarah"),
            age: 30,
            gender: Gender::Agender,
            alignment: Alignment::Good,
            damage: 10,
            health: 100,
            dead: false
        },
        Person {
            name: String::from("Henry"),
            age: 30,
            gender: Gender::Agender,
            alignment: Alignment::Good,
            damage: 10,
            health: 100,
            dead: false
        },
    ];

    let mut blue: [Person; 2] = [list[0], list[1]];
    let mut red: [Person; 2] = [list[0], list[1]];

    let mut alive_blue: Vec<&mut Person> = blue.iter_mut().filter(|person| !person.dead).collect();
    let mut alive_red: Vec<&mut Person> = red.iter_mut().filter(|person| !person.dead).collect();
    
}
