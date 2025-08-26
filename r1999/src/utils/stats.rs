pub struct Chara {
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
