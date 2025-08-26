use crate::utils::stats::Enemy;
use crate::utils::stats::Chara;
use crate::utils::stats::Attack;

pub fn damage(character: &Chara, enemy: &Enemy, attack: &Attack, multiplier: u32) -> u32 {
    
    let base :u32 = 30;
    
    //dmgdealt
    let mut dmgdealt:u32;
    dmgdealt = 100+character.dmgbonus-enemy.dmgred;
    //lower limit of .3
    if dmgdealt <= 30 {dmgdealt = base;}
    
    println!("damage bonus: {}",dmgdealt);

    //might
    let mut might :u32 = 1;
    if attack.incan == true {
        might += character.incan;
    }
    if attack.ult == true {
        might += character.ult;
    }
    
    println!("might:{}", might);

    //att
    let def :u32;
    if attack.mental == true {def = enemy.mdef;
    } else {def = enemy.rdef;}
    let att :u32= character.atk-(def*(100-character.penrate)/100);

    println!("Attack: {}", att);

    let mut damage = multiplier*att*dmgdealt*might/100/100;
    
    //afflatus
    if attack.strong == true {damage = damage * 130 / 100;}

    //crit
    let damagecrit :u32 = (character.critdmg - enemy.critdef) * damage/100;
    
    //final dmg
    

    println!("crit rate:{}%", character.critrate-enemy.critres);  
    println!("if crit: {}", damagecrit);

    return damage;
    
}

