use crate::Enemy;
use crate::Chara;
use crate::Attack;

pub fn damage(character: Chara, enemy: Enemy, attack: Attack, multiplier: u32) {
    
    //dmgdealt
    let dmgdealt: u32;
    //lower limit of .3
    if character.dmgbonus+70 <= enemy.dmgred {dmgdealt = 30;}
    else {dmgdealt = 100+character.dmgbonus-enemy.dmgred;}
    println!("\ndamage bonus: {}",dmgdealt-100);

    //might
    let mut might: u32 = 100;
    if attack.incan == true {
        might += character.incan;
    }
    if attack.ult == true {
        might += character.ult;
    }
    println!("Incan/Ult might:{}", might);

    //att
    let def: u32;
    let att: u32;
    if attack.mental == true {def = enemy.mdef;
    } else {def = enemy.rdef;}
    if def*(100-character.penrate)/100 >= character.atk {
        att = 0;
    } else {
        att = character.atk-(def*(100-character.penrate)/100); }

    println!("Attack: {}", att);

    let mut damage = (multiplier*att/100*dmgdealt/100*might/100) as f64;
    
    //afflatus
    if attack.strong == true {damage = damage * 1.3;}

    //crit
    let mut damagecrit = f64::from(character.critdmg - enemy.critdef);
    damagecrit = damagecrit * damage/100.0;
    

    println!("crit rate:{}%", character.critrate-enemy.critres);  
    println!("if crit: {}", damagecrit);

    println!("no crit: {}", damage);
    
}

