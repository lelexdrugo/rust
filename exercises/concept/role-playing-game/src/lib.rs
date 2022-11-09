// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Option::from( match self.level {
                0..=9 => Player { health: 100, mana: Option::None, level: self.level },
                10.. => Player { health: 100, mana: Option::Some(100), level: self.level },
            }),
            _ => Option::None,//Player { ..*self },
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        return if(self.mana.is_none()){
            self.health -= min(mana_cost, self.health);
            0
        }
        else if self.mana.is_some() && self.mana.unwrap()<=mana_cost {
            0
        } else {
            let mut mana = self.mana.unwrap();
            mana -= mana_cost;
            self.mana = Option::Some(mana);
            2 * mana_cost
        }
    }
}

