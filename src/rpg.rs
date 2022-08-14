#[derive(Debug, PartialEq)]
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            if self.level < 10 {
                return Some(Player {
                    health: 100,
                    mana: None,
                    level: self.level,
                });
            } else {
                return Some(Player {
                    health: 100,
                    mana: Some(self.mana.unwrap()),
                    level: self.level,
                });
            }
        } else {
            return None;
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health -= mana_cost;
                0
            }
            Some(m) => {
                if m < mana_cost {
                    0
                } else {
                    self.mana = Some(self.mana.unwrap() - mana_cost);
                    mana_cost * 2
                }
            }
        }
    }
}

#[cfg(test)]
#[test]
fn revives_dead() {
    let dead_player = Player {
        health: 0,
        mana: None,
        level: 2,
    };
    assert_eq!(
        dead_player.revive(),
        Some(Player {
            health: 100,
            mana: None,
            level: 2
        })
    );
}

#[test]
fn does_not_revive_alive() {
    let alive_player = Player {
        health: 1,
        mana: Some(15),
        level: 11,
    };
    assert_eq!(alive_player.revive(), None)
}

#[test]
fn does_not_cast_spell_without_mana() {
    let mut not_a_wizard_yet = Player {
        health: 79,
        mana: None,
        level: 9,
    };
    assert_eq!(not_a_wizard_yet.cast_spell(5), 0);
    assert_eq!(not_a_wizard_yet.health, 74);
    assert_eq!(not_a_wizard_yet.mana, None);
}

#[test]
fn does_not_cast_spell() {
    let mut low_mana_wizard = Player {
        health: 93,
        mana: Some(3),
        level: 12,
    };
    assert_eq!(low_mana_wizard.cast_spell(10), 0);
    assert_eq!(low_mana_wizard.health, 93);
    assert_eq!(low_mana_wizard.mana, Some(3));
}

#[test]
fn does_cast_spell() {
    let mut wizard = Player {
        health: 123,
        mana: Some(30),
        level: 18,
    };
    assert_eq!(wizard.cast_spell(10), 20);
    assert_eq!(wizard.health, 123);
    assert_eq!(wizard.mana, Some(20));
}
