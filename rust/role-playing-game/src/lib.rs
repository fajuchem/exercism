// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            health if health <= 0 => {
                return Some(Player {
                    health: 100,
                    mana: if self.level >= 10 { Some(100) } else { None },
                    level: self.level,
                })
            }
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if mana.le(&mana_cost) {
                    return 0;
                } else {
                    self.mana = Some(mana - mana_cost);
                    return mana_cost * 2;
                }
            }
            None => {
                if mana_cost > self.health {
                    self.health = 0;
                } else {
                    self.health = self.health - mana_cost;
                }
                return 0;
            }
        }
    }
}
