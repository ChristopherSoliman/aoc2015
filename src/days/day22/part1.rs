use std::collections::BinaryHeap;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct Character {
    hp: usize,
    mana: usize,
    damage: usize,
}

#[derive(Debug, Hash, Copy, PartialEq, Eq, Clone)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    pub fn get_cost(&self) -> usize {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

#[derive(Debug, Hash, Copy, PartialEq, Eq, Clone)]
struct CastSpell {
    name: Spell,
    turns_remaining: usize,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct State {
    spent: usize,
    boss: Character,
    hero: Character,
    spells: Vec<CastSpell>,
}

impl State {
    pub fn get_spell_effects(&mut self) -> (usize, usize) {
        let mut damage = 0;
        let mut armor = 0;
        for spell in &mut self.spells {
            match spell.name {
                Spell::Shield => armor += 7,
                Spell::Recharge => self.hero.mana += 101,
                Spell::Poison => damage += 3,
                _ => {}
            }
            spell.turns_remaining -= 1;
        }

        self.spells.retain(|s| s.turns_remaining > 0);
        (damage, armor)
    }
    pub fn play_turn(&mut self, spell: &Spell) -> usize {
        self.hero.mana -= spell.get_cost();
        match spell {
            Spell::MagicMissile => return 4,
            Spell::Drain => {
                self.hero.hp += 2;
                return 2;
            }
            Spell::Shield => {
                self.spells.push(CastSpell {
                    name: spell.clone(),
                    turns_remaining: 5,
                });
                return 0;
            }
            Spell::Poison => {
                self.spells.push(CastSpell {
                    name: spell.clone(),
                    turns_remaining: 6,
                });
                return 0;
            }
            Spell::Recharge => {
                self.spells.push(CastSpell {
                    name: spell.clone(),
                    turns_remaining: 5,
                });
                return 0;
            }
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.spent.cmp(&self.spent))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.spent.cmp(&self.spent)
    }
}

const SPELLS: [Spell; 5] = [
    Spell::MagicMissile,
    Spell::Drain,
    Spell::Shield,
    Spell::Poison,
    Spell::Recharge,
];

pub fn run() -> usize {
    djikstras()
}

fn djikstras() -> usize {
    let hero = Character {
        hp: 50,
        mana: 500,
        damage: 0,
    };

    let boss = Character {
        hp: 55,
        mana: 0,
        damage: 8,
    };

    let mut q = BinaryHeap::from(vec![State {
        spent: 0,
        hero: hero.clone(),
        boss: boss.clone(),
        spells: vec![],
    }]);

    while let Some(state) = q.pop() {
        if state.boss.hp == 0 {
            return state.spent;
        }
        'spells_loop: for spell in SPELLS {
            let mut state = state.clone();
            if spell.get_cost() > state.hero.mana {
                continue;
            }
            for sp in &state.spells {
                if sp.name == spell && sp.turns_remaining != 1 {
                    continue 'spells_loop;
                }
            }
            simulate(&mut state, &spell);
            if state.hero.hp == 0 {
                continue;
            }
            q.push(State {
                boss: state.boss,
                hero: state.hero,
                spent: state.spent + spell.get_cost(),
                spells: state.spells.clone(),
            });
        }
    }
    0
}

fn simulate(state: &mut State, spell: &Spell) {
    for i in 0..2 {
        let (hero_damage, hero_armor) = state.get_spell_effects();
        if hero_damage >= state.boss.hp {
            state.boss.hp = 0;
            return;
        }
        state.boss.hp -= hero_damage;

        if i == 0 {
            let turn_damage = state.play_turn(spell);
            if turn_damage >= state.boss.hp {
                state.boss.hp = 0;
                return;
            }
            state.boss.hp -= turn_damage;
        } else {
            let b_damage = std::cmp::max(state.boss.damage as i32 - hero_armor as i32, 1) as usize;

            if b_damage >= state.hero.hp {
                state.hero.hp = 0;
                return;
            }
            state.hero.hp -= b_damage;
        }
    }
}
