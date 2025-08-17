use bytemon_bsys::rpg_base_system::battle_essentials::Battler;
use bytemon_bsys::rpg_base_system::battle_essentials::EType;
use bytemon_bsys::rpg_base_system::battle_essentials::SpecialAttack;

fn main() {
    let mut b1 = Battler::default();
    let mut b2 = Battler::default();

    b1.characteristics.type_alignment = EType::Vaccine;
    b1.name = "Agumon".to_owned();
    b2.characteristics.type_alignment = EType::Data;
    b2.name = "Patamon".to_owned();
    b1.basic_attack(&mut b2);

    b1.special_attacks.push(SpecialAttack::wolkenapalm1_f_p());
    println!("{b1:?} \n VS \n {b2:?}");
    println!("Hello, world!");

    println!("{} wins!", battle_start(&mut b1, &mut b2));
}

fn battle_start(b1: &mut Battler, b2: &mut Battler) -> String {
    let mut b1_timer = 0;
    let mut b2_timer = 0;

    loop {
        println!(
            "B1: HP:{:0>6} TIMER: {:0>3}\nB2: HP:{:0>6} TIMER: {:0>3}",
            b1.status.hp, b1_timer, b2.status.hp, b2_timer
        );
        b1_timer += b1.status.agility;
        b2_timer += b2.status.agility;

        if b1_timer > 100 {
            b1.basic_attack(b2);
            b1_timer = 0
        }
        if b2_timer > 100 {
            b2.basic_attack(b1);
            b2_timer = 0
        }

        if b1.status.hp <= 0 {
            return String::from(&b2.name);
        }
        if b2.status.hp <= 0 {
            return String::from(&b1.name);
        }
    }
}
