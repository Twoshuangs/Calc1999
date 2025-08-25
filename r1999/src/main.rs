mod utils;

/*struct Chara {
    atk: u32,
    hp: u32,
    rdef: u32,
    mdef: u32,
    crittech: u32,
    critrate: f64,
    critdmg: f64,
    dmgbonus: u32,
    incan: u32,
    ult: u32,
    penrate: u32,
}

struct Enemy {
    rdef: u32,
    mdef: u32,
    critres: u32,
    critdef: u32,
    dmgred: u32,
}
*/

fn main() {
    //temp
    let melania = utils::stats::Chara{
        atk: 2000,
        hp: 1000,
        rdef: 100,
        mdef: 100,
        crittech: 100,
        critrate: 50.0,
        critdmg: 150.0,
        dmgbonus: 10,
        incan: 10,
        ult: 10,
        penrate: 10,
    };
    let carbuncle = utils::stats::Enemy {
        rdef: 10,
        mdef: 10,
        critres: 10,
        critdef: 10,
        dmgred: 10,
    };
    
    let multiplier: u32 = 200;
    let dmg = utils::calc::damage(&melania, &carbuncle, multiplier);
    println!("Damage: {}",dmg);
}

