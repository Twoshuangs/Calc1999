use std::fs;
use std::io;
mod calc;

//to-do: [y/n] input loop, overflow, calc numbers

#[derive(serde::Deserialize, Clone)]
pub struct Chara {
    pub name: String,
    pub atk: u32,
    pub critrate: u32,
    pub critdmg: u32,
    pub dmgbonus: u32,
    pub incan: u32,
    pub ult: u32,
    pub penrate: u32,
}

pub struct Enemy {
    pub rdef: u32,
    pub mdef: u32,
    pub critres: u32,
    pub critdef: u32,
    pub dmgred: u32,
}

pub struct Attack {
    pub incan: bool,
    pub ult: bool,
    pub strong: bool, 
    pub mental: bool, 
}

fn load_chara(path: &str) -> Vec<Chara> {
    let data = fs::read_to_string(path).expect("Unable to read file");
    serde_json::from_str::<Vec<Chara>>(&data).expect("JSON parsing failed")
}

fn main() {
    //temp
    let carbuncle = Enemy {
        rdef: 765,
        mdef: 637,
        critres: 0,
        critdef: 19,
        dmgred: 15,
    };

    let atktype = atkconf();
    let multiplier = get_u32("Multiplier");
    let characters = load_chara("char.json");
    let x: u32 = characters.len() as u32;

    println!("Characters:");
    for (i, c) in characters.iter().enumerate() {
        println!("{} {}", i, c.name);
    }
    let mut chara = get_u32 ("Choice"); 
    
    loop {

        if chara <= x {
            break;
        } else {
            println!("Invalid choice");
            chara = get_u32 ("Choice");
        }
    }
    let chara_u: usize = chara as usize;
    let character = characters[chara_u].clone();

    calc::damage(character, carbuncle, atktype, multiplier);
}

fn atkconf () -> Attack {
    let incan = get_input("Incantation");
    let ult = get_input("Ultimate");
    let strong = get_input("Stronger Afflatus");
    let mental  = get_input("Mental damage type");
    Attack {
        incan,
        ult,
        strong,
        mental,
    }        
}

fn get_input(prompt: &str) -> bool{
    println!("\n{}: [y/n]", prompt);
    let i; 
    'check: loop{ 

        let mut input = String::new();        
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let x = input.trim().to_string();

        if x == "y"{
            i = true;
            break 'check;
        } else if x == "n" {
            i = false;
            break 'check;
        } else {
            println!("Please choose between [y/n]");
            continue;
        }
    }
    return i;
}

fn get_u32(prompt: &str) -> u32{
    println!("\n{}: ",prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input: u32 = input.trim().parse().expect("Not a valid number");
    return input;
}
