#[repr(u8)]
#[derive(Debug)]
pub enum EType {
    Free = 1,
    Vaccine = 2,
    Data = 3,
    Virus = 4,
}

impl EType {
    pub fn get_advantage(&self, target: &EType) -> f32 {
        match (self, target) {
            (EType::Data, EType::Vaccine) => 2.0,
            (EType::Vaccine, EType::Virus) => 2.0,
            (EType::Virus, EType::Data) => 2.0,

            (EType::Vaccine, EType::Data) => 0.5,
            (EType::Virus, EType::Vaccine) => 0.5,
            (EType::Data, EType::Virus) => 0.5,
            _ => 1.0,
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum EAttribute {
    Fire = 1,
    Water = 2,
    Plant = 3,
    Eletric = 4,
    Earth = 5,
    Wind = 6,
    Light = 7,
    Dark = 8,
    Neutral = 9,
}

impl EAttribute {
    pub fn get_advantage(&self, target: &EAttribute) -> f32 {
        match (self, target) {
            (EAttribute::Fire, EAttribute::Plant) => 1.5,
            (EAttribute::Plant, EAttribute::Water) => 1.5,
            (EAttribute::Water, EAttribute::Fire) => 1.5,

            (EAttribute::Eletric, EAttribute::Wind) => 1.5,
            (EAttribute::Wind, EAttribute::Earth) => 1.5,
            (EAttribute::Earth, EAttribute::Eletric) => 1.5,

            (EAttribute::Light, EAttribute::Dark) => 1.5,
            (EAttribute::Dark, EAttribute::Light) => 1.5,

            (EAttribute::Plant, EAttribute::Fire) => 0.75,
            (EAttribute::Water, EAttribute::Plant) => 0.75,
            (EAttribute::Fire, EAttribute::Water) => 0.75,

            (EAttribute::Wind, EAttribute::Eletric) => 0.75,
            (EAttribute::Earth, EAttribute::Wind) => 0.75,
            (EAttribute::Eletric, EAttribute::Earth) => 0.75,
            _ => 1.0,
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum ELevel {
    Rookie = 1,
    Champion = 2,
    Ultimate = 3,
    Mega = 4,
    Ultra = 5,
    Armor = 6,
}

pub enum ECondition {}

pub enum EBuff {}

#[derive(Debug)]
pub struct Battler {
    pub id: u8,
    pub level: u8,
    pub name: String,
    pub status: Status,
    pub characteristics: Characteristics,
    total_exp: u32,
    next_lv_exp: u16,
}

impl Default for Battler {
    fn default() -> Self {
        Self {
            id: 1,
            level: 1,
            name: "battler".to_owned(),
            status: Status::default(),
            characteristics: Characteristics::new(EType::Free, EAttribute::Neutral, ELevel::Rookie),
            total_exp: 0,
            next_lv_exp: 10,
        }
    }
}

impl Battler {
    pub fn new(
        id: u8,
        level: u8,
        name: String,
        status: Status,
        characteristics: Characteristics,
        total_exp: u32,
        next_lv_exp: u16,
    ) -> Self {
        Self {
            id,
            level,
            name,
            status,
            characteristics,
            total_exp,
            next_lv_exp,
        }
    }

    pub fn basic_attack(&self, b: &mut Battler) {
        let t_advantage = self
            .characteristics
            .type_alignment
            .get_advantage(&b.characteristics.type_alignment);

        let a_advantage = self
            .characteristics
            .attribute
            .get_advantage(&b.characteristics.attribute);

        let advantage = t_advantage * a_advantage;
        let dmg_calc = (self.status.attack / b.status.defense * 12 + 3) as f32 * advantage;
        b.take_damage(dmg_calc.round() as i16);
    }

    pub fn take_damage(&mut self, d: i16) {
        self.status.take_damage(d);
    }
}

#[derive(Debug)]
pub struct Status {
    pub hp: i16,
    pub mp: i16,
    pub attack: i16,
    pub defense: i16,
    pub wisdom: i16,
    pub agility: i16,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            hp: 100,
            mp: 10,
            attack: 10,
            defense: 10,
            wisdom: 10,
            agility: 10,
        }
    }
}

impl Status {
    pub fn take_damage(&mut self, d: i16) {
        self.hp -= d
    }
}

#[derive(Debug)]
pub struct Characteristics {
    pub type_alignment: EType,
    pub attribute: EAttribute,
    evo_level: ELevel,
}

impl Characteristics {
    pub fn new(type_alignment: EType, attribute: EAttribute, evo_level: ELevel) -> Self {
        Self {
            type_alignment,
            attribute,
            evo_level,
        }
    }
}
