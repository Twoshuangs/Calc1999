use std::fs;
//use std::fs::OpenOptions;
use std::io;
use std::path::PathBuf;
mod calc;
extern crate dirs;

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

const DEFAULT_CHAR: &'static str = r#"[{ "name": "melania", "atk": 1929, "critrate": 51,  "critdmg": 185, "dmgbonus": 35, "incan": 0, "ult": 58, "penrate": 0 }, { "name": "semmelweis", "atk": 0, "critrate":10, "critdmg": 200, "dmgbonus": 80, "incan": 10, "ult": 0, "penrate": 0} ] "# ; 

fn load_chara(path: PathBuf) -> Vec<Chara> {
    println!("Loading characters...");
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

    check_config();
    let atktype = atkconf();
    let multiplier = get_u32("Multiplier");
    
    let mut conf_loc: PathBuf = dirs::config_dir().expect("Can't find config location");
    conf_loc.push("calc1999");
    conf_loc.push("char.json");
    
    let characters = load_chara(conf_loc);
    let x: u32 = characters.len() as u32;

    println!("\nCharacters:");
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

fn check_config(){
    let mut conf_loc: PathBuf = dirs::config_dir().expect("Can't find config location");
    conf_loc.push("calc1999");
    conf_loc.push("char.json");
    match fs::exists(conf_loc) {
        Ok(true) => println!("Character list found"),
        Ok(false) => generate_config(),
        Err(_) => println!("Can't check for exisiting character list"),
    };
    
}

fn generate_config(){
    let mut conf_loc: PathBuf = dirs::config_dir().expect("Can't find config location");
    conf_loc.push("calc1999");
    println!("Generating default characters...");
    match fs::create_dir(&conf_loc){
        Ok(()) => println!("Directory created"),
        Err(e) => println!("Failed to create directory: {:?}",e.kind()),
    };
    conf_loc.push("char.json");
    match fs::write(conf_loc,DEFAULT_CHAR) {
        Ok(()) => println!("Characters loaded"),
        Err(_) => println!("Can't write file"),
    };
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
