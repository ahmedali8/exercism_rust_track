use std::cmp;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        // 1st way
        // match self.health {
        //     0 if self.level >= 10 => {
        //         Some(Player {
        //             health: 100,
        //             mana: Some(100),
        //             level: self.level,
        //         })
        //     },
        //     0 => {
        //         Some(Player {
        //             health: 100,
        //             mana: None,
        //             level: self.level,
        //         })
        //     }
        //     _ => None,
        // }

        // 2nd way
        (self.health == 0).then(|| Player {
            health: 100,
            mana: self.level.ge(&10).then(|| 100),
            level: self.level, 
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) if mana >= mana_cost => {
                self.mana = Some(mana - mana_cost);
                mana_cost * 2
            },
            None => {
                self.health -= cmp::min(self.health, mana_cost);
                0
            },
            _ => {
                0
            }
        }
    }
}
