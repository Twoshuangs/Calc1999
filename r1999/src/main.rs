mod utils;

fn main() {
    //temp
    let melania = utils::stats::Chara{
        atk: 1929,
        critrate: 51,
        critdmg: 185,
        dmgbonus: 55,
        incan: 0,
        ult: 58,
        penrate: 0,
    };
    let carbuncle = utils::stats::Enemy {
        rdef: 765,
        mdef: 637,
        critres: 0,
        critdef: 19,
        dmgred: 15,
    };
    let kick = utils::stats::Attack {
        incan: true, 
        ult: false,
        strong: false, 
        mental: true, 
    };

    let multiplier: u32 = 220;
    let dmg = utils::calc::damage(&melania, &carbuncle, &kick, multiplier);
    println!("Damage: {}",dmg);
}

