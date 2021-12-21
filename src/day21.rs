use crate::utils::DayResult;

use std::str::{FromStr, Lines};

pub(crate) fn main(stdin: Lines) -> DayResult {
    let mut players = Vec::with_capacity(2);
    for l in stdin {
        let pos = u16::from_str(l.split_at(28).1)
            .map_err(|e| format!("Expected to parse number: {}", e))?;
        players.push(Player { pos, score: 0 })
    }
    if players.len() != 2 {
        return Err("Expected to get 2 players".to_string());
    }
    let mut die = PracticeDie { rolls: 0 };
    let winplayer = 'runner: loop {
        for (i, player) in &mut players.iter_mut().enumerate() {
            player.take_turn(&mut die);
            if player.score >= 1000 {
                break 'runner i;
            }
        }
    };
    let loseplayer = players.get(1 - winplayer).unwrap();
    Ok((
        Ok((loseplayer.score as u32 * die.rolls as u32).to_string()),
        Err("not implemented yet".to_string()),
    ))
}

trait Die {
    fn roll(&mut self) -> u16;
    fn get_rolls_number(&self) -> u16;
}

struct PracticeDie {
    rolls: u16,
}
impl Die for PracticeDie {
    fn roll(&mut self) -> u16 {
        self.rolls += 1;
        (self.rolls - 1) % 100 + 1
    }
    fn get_rolls_number(&self) -> u16 {
        self.rolls
    }
}

#[derive(Debug)]
struct Player {
    pos: u16,
    score: u16,
}

impl Player {
    fn take_turn<T: Die>(&mut self, die: &mut T) {
        self.pos = (self.pos + die.roll() + die.roll() + die.roll() - 1) % 10 + 1;
        self.score += self.pos;
    }
}
