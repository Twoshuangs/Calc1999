use std::cmp::Ordering;
use crate::utils::stats::Enemy;
use crate::utils::stats::Chara;

pub fn damage(character: &Chara, enemy: &Enemy, multiplier: u32) -> u32 {
    
    let base :u32 = 40;
    let dmgdealt:u32;

    //checking if damage has hit lower limit
    match base.cmp(&(100+character.dmgbonus-enemy.dmgred)) {
        Ordering::Greater => {
            dmgdealt = base;
        }
        Ordering::Less | Ordering::Equal => {
            dmgdealt = 100+character.dmgbonus-enemy.dmgred;
        }
    }

    (multiplier/100) * (character.atk - enemy.mdef) * dmgdealt/100 * (100 + character.incan)/100

}

