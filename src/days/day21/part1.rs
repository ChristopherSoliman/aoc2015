#[derive(Debug, Clone, Eq, PartialEq)]
struct Character {
    hp: usize,
    damage: usize,
    armor: usize,
}
#[derive(Debug, Clone, Eq, PartialEq)]
struct StoreItem {
    name: String,
    cost: usize,
    damage: usize,
    armor: usize,
}

const BOSS: Character = Character {
    hp: 100,
    armor: 2,
    damage: 8,
};

pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let input = input.split("\r\n\r\n");

    let mut weapons = vec![];
    let mut armors = vec![];
    let mut rings = vec![];

    for (i, store_type) in input.enumerate() {
        for line in store_type.lines().skip(1) {
            let a = line.trim().split_whitespace().collect::<Vec<_>>();
            let item = StoreItem {
                name: a[0].to_string(),
                cost: a[1].parse().unwrap(),
                damage: a[2].parse().unwrap(),
                armor: a[3].parse().unwrap(),
            };
            match i {
                0 => weapons.push(item),
                1 => armors.push(item),
                2 => rings.push(item),
                _ => unreachable!("invalid item type"),
            }
        }
    }
    let none_item = StoreItem {
        name: "None".to_string(),
        cost: 0,
        damage: 0,
        armor: 0,
    };
    armors.push(none_item.clone());
    rings.push(none_item.clone());

    let mut winning = usize::MAX;

    for weapon in &weapons {
        for armor in &armors {
            for ring1 in &rings {
                for ring2 in &rings {
                    if ring1.name != "None" && ring1 == ring2 {
                        continue;
                    }
                    let total_cost = armor.cost + weapon.cost + ring1.cost + ring2.cost;
                    let character = Character {
                        armor: armor.armor + ring1.armor + ring2.armor,
                        damage: weapon.damage + ring1.damage + ring2.damage,
                        hp: 100,
                    };
                    if simulate(&character) && total_cost < winning {
                        winning = total_cost;
                    }
                }
            }
        }
    }
    winning
}

fn simulate(character: &Character) -> bool {
    let c_damage = std::cmp::max(character.damage as i32 - BOSS.armor as i32, 1);
    let b_damage = std::cmp::max(BOSS.damage as i32 - character.armor as i32, 1);
    let c_rounds = f32::ceil(BOSS.hp as f32 / c_damage as f32);
    let b_rounds = f32::ceil(character.hp as f32 / b_damage as f32);
    c_rounds <= b_rounds
}
