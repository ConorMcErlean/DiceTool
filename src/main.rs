use core::fmt;
use rand::Rng;
use std::env;

enum AdvType {
    Advantage,
    Disadvantage,
    Standard,
}

#[derive(Debug)]
enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

struct Roll {
    number: u8,
    dice_type: Dice,
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rolling {} x {:?}", self.number, self.dice_type)
    }
}

impl Roll {
    fn generate_roll(&self) -> Vec<u8> {
        let mut rolls = vec![];
        for _ in 1..self.number {
            rolls.push(generate(&self.dice_type));
        }
        rolls
    }
}

fn generate(dice: &Dice) -> u8 {
    let range = match dice {
        Dice::D4 => 4,
        Dice::D6 => 6,
        Dice::D8 => 8,
        Dice::D10 => 10,
        Dice::D12 => 12,
        Dice::D20 => 20,
    };
    let mut rng = rand::thread_rng();
    rng.gen_range(1..range + 1)
}

fn parse_roll(input: &str) -> Roll {
    let input = input.to_lowercase();
    let parts: Vec<&str> = input.split("d").collect();

    Roll {
        number: match parts[0] {
            "" => 1,
            _ => parts[0].parse::<u8>().unwrap(),
        },
        dice_type: Dice::D20,
    }
}

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let num_of_args = args.len();
    let modifier = if num_of_args > 2 {
        &args[num_of_args - 1]
    } else {
        "0"
    };

    let roll = parse_roll(&args[1]);
    let result = roll.generate_roll();
    println!("{} with a {} modifier", roll, modifier);
    // Input roll {Number}d{20} {adv/dis} {+mod}
    //
}
