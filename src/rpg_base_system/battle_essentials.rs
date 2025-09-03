#[repr(u8)]
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub enum ELevel {
    Rookie = 1,
    Champion = 2,
    Ultimate = 3,
    Mega = 4,
    Ultra = 5,
    Armor = 6,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ECondition {
    Defending,
}

#[derive(Debug, Clone)]
pub enum EBuff {}

#[derive(Debug, Clone)]
pub enum ETypeAttack {
    Physical,
    Wisdom,
}

#[derive(Debug, Clone)]
pub enum ETypeTarget {
    OneEnemy,
    AllEnemies,
    OneAlly,
    AllAllies,
}

#[derive(Debug, Clone)]
pub struct SpecialAttack {
    pub attribute: EAttribute,
    pub cost: i16,
    pub damage_value: i16,
    pub cure_value: i16,
    pub condition_effect: Option<(ECondition, i8)>,
    pub buff_effect: Option<(EBuff, i8)>,
    pub type_attack: ETypeAttack,
    pub type_target: ETypeTarget,
}

impl SpecialAttack {
    fn new_simple_attack(
        attribute: EAttribute,
        type_attack: ETypeAttack,
        type_target: ETypeTarget,
        damage_value: i16,
        cost: i16,
    ) -> Self {
        Self {
            attribute,
            type_attack,
            type_target,
            damage_value,
            cost,
            cure_value: 0,
            condition_effect: None,
            buff_effect: None,
        }
    }

    pub fn wolkenapalm1_f_p() -> Self {
        SpecialAttack::new_simple_attack(
            EAttribute::Fire,
            ETypeAttack::Physical,
            ETypeTarget::OneEnemy,
            65,
            3,
        )
    }
}

#[derive(Debug, Clone)]
pub struct Battler {
    pub id: u8,
    pub level: u8,
    pub name: String,
    pub conditions: Vec<(ECondition, i8)>,
    pub buffs: Vec<EBuff>,
    pub special_attacks: Vec<SpecialAttack>,
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
            conditions: vec![],
            buffs: vec![],
            special_attacks: vec![],
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
        conditions: Vec<(ECondition, i8)>,
        buffs: Vec<EBuff>,
        special_attacks: Vec<SpecialAttack>,
        characteristics: Characteristics,
        total_exp: u32,
        next_lv_exp: u16,
    ) -> Self {
        Self {
            id,
            level,
            name,
            status,
            conditions,
            buffs,
            special_attacks,
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

    pub fn special_attack_start(&self, sp: SpecialAttack, targets: &mut Vec<Battler>) {
        match sp.type_target {
            ETypeTarget::OneEnemy => (),
            ETypeTarget::AllEnemies => (),
            _ => (),
        }
    }

    pub fn take_damage(&mut self, d: i16) {
        if self
            .conditions
            .iter()
            .any(|(x, y)| x == &ECondition::Defending && *y > 0)
        {
            self.status.take_damage(d / 2);
        } else {
            self.status.take_damage(d);
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Characteristics {
    pub type_alignment: EType,
    pub attribute: EAttribute,
    pub evo_level: ELevel,
}

impl Default for Characteristics {
    fn default() -> Self {
        Self {
            type_alignment: EType::Free,
            attribute: EAttribute::Neutral,
            evo_level: ELevel::Rookie,
        }
    }
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

#[derive(Clone)]
pub struct BattlerTimer {
    turn_cooldown: i16,
    battler: Battler,
}

#[derive(Clone)]
pub struct BattleTeam<const fr_limit: usize> {
    frontrow: [BattlerTimer; fr_limit],
    backrow: Option<[BattlerTimer; 3]>,
}

pub struct BattleManager {
    allies: BattleTeam<3>,
    enemies: BattleTeam<3>,
    turn_count: u8,
    ids_turn_order_preview: [u8; 15],
}

impl BattleManager {
    fn new_define_teams(allies: BattleTeam<3>, enemies: BattleTeam<3>) -> Self {
        Self {
            allies,
            enemies,
            turn_count: 0,
            ids_turn_order_preview: [0; 15],
        }
    }

    fn get_all_battlers(self) -> Vec<BattlerTimer> {
        let mut all_battlers = self.allies.frontrow.to_vec();
        all_battlers.extend(self.enemies.frontrow);

        all_battlers
    }

    fn calculate_turn_order(self) -> Option<BattlerTimer> {
        let mut all_battlers = self.get_all_battlers();
        loop {
            all_battlers
                .iter_mut()
                .for_each(|x| x.turn_cooldown += x.battler.status.agility);

            if let Some(battler) = all_battlers.iter_mut().find(|x| x.turn_cooldown > 100) {
                battler.turn_cooldown = -10;
                break Some(battler.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests_battle_manager {

    use super::*;
    #[test]
    fn test_turn_calc() {
        let mut allies = BattleTeam {
            frontrow: [
                BattlerTimer {
                    battler: Battler::default(),
                    turn_cooldown: 0,
                },
                BattlerTimer {
                    battler: Battler::default(),
                    turn_cooldown: 0,
                },
                BattlerTimer {
                    battler: Battler::default(),
                    turn_cooldown: 0,
                },
            ],
            backrow: None,
        };

        allies.frontrow[0].battler.id = 2;
        allies.frontrow[0].battler.status.agility = 5;
        allies.frontrow[1].battler.id = 3;
        allies.frontrow[1].battler.status.agility = 8;
        allies.frontrow[2].battler.id = 4;
        allies.frontrow[2].battler.status.agility = 12;

        let mut enemies = BattleTeam {
            frontrow: [
                BattlerTimer {
                    battler: Battler::default(),
                    turn_cooldown: 0,
                },
                BattlerTimer {
                    battler: Battler::default(),
                    turn_cooldown: 0,
                },
                BattlerTimer {
                    battler: Battler::default(),
                    turn_cooldown: 0,
                },
            ],
            backrow: None,
        };

        let bm = BattleManager::new_define_teams(allies, enemies);

        let battler_result = bm.calculate_turn_order();

        if let Some(b) = battler_result {
            assert_eq!(b.battler.id, 4);
            assert!(b.turn_cooldown < 0);
        }
    }
}
