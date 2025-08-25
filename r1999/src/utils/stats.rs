pub struct Chara {
    pub atk: u32,
    pub hp: u32,
    pub rdef: u32,
    pub mdef: u32,
    pub crittech: u32,
    pub critrate: f64,
    pub critdmg: f64,
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
