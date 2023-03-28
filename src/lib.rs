use std::default;

use log::info;

pub trait Score {
    fn score(&self) -> i32;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameBonusType {
    Spare,
    Strike,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Frame {
    pub first_roll_pins: i32,
    pub second_roll_pins: Option<i32>,
    pub bonus: Option<FrameBonusType>,
}

impl Frame {
    fn rolls_score(&self) -> i32 {
        if self.second_roll_pins.is_none() {
            return self.first_roll_pins;
        }
        self.first_roll_pins + self.second_roll_pins.unwrap()
    }
}

impl Score for Frame {
    fn score(&self) -> i32 {
        let mut score = self.rolls_score();
        if let Some(bonus) = self.bonus {
            match bonus {
                FrameBonusType::Spare => score += self.first_roll_pins,
                FrameBonusType::Strike => {
                    score += self.first_roll_pins;
                    if let Some(second_roll) = self.second_roll_pins {
                        score += second_roll;
                    }
                }
            }
        }

        info!("Score for frame is {} ", score);
        score
    }
}

#[derive(Default, Debug)]
pub struct Game {
    frames: [Frame; 10],
    current_frame_index: usize,
    current_roll_index: usize,
    bonus_tenth_frame_third_roll: Option<i32>,
}

impl Game {
    pub fn roll(&mut self, pins: i32) {
        // update game frames
        let mut frame = &mut self.frames[self.current_frame_index];
        let next_frame_index = if self.current_frame_index + 1 < 10 {
            Some(self.current_frame_index + 1)
        } else {
            None
        };
        let mut bonus: Option<FrameBonusType> = None;
        match self.current_roll_index {
            0 => {
                frame.first_roll_pins = pins;

                if pins == 10 {
                    bonus = Some(FrameBonusType::Strike);
                }
            }
            1 => {
                frame.second_roll_pins = Some(pins);
                if pins + frame.first_roll_pins == 10 {
                    bonus = Some(FrameBonusType::Spare);
                }
            }
            2 => {
                // bonus roll
                self.bonus_tenth_frame_third_roll = Some(pins)
            }
            _ => {}
        }

        if let Some(i) = next_frame_index {
            self.frames[i].bonus = bonus;
        }

        self.set_next_indices(bonus);
    }

    fn set_next_indices(&mut self, bonus: Option<FrameBonusType>) {
        match self.current_roll_index {
            0 => {
                let is_strike = match bonus {
                    None => false,
                    Some(x) => (|b| b == FrameBonusType::Strike)(x),
                };

                if !is_strike {
                    self.current_roll_index = 1;
                } else {
                    self.current_frame_index += 1;
                    self.current_roll_index = 0;
                }
            }
            1 => {
                if self.current_frame_index != 9 {
                    self.current_roll_index = 0;
                    self.current_frame_index += 1;
                } else {
                    if let Some(_) = bonus {
                        self.current_roll_index = 2;
                    }
                }
            }
            _ => {}
        }
    }
}

impl Score for Game {
    fn score(&self) -> i32 {
        let mut total_score = 0i32;
        for frame in self.frames {
            total_score += frame.score()
        }
        if let Some(bonus_last_roll_pins) = self.bonus_tenth_frame_third_roll {
            total_score += bonus_last_roll_pins;
        }
        total_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let new_game = Game::default();
        assert_eq!(new_game.score(), 0);
    }

    #[test]
    fn it_should_have_score_of_nine_after_rolls_of_4_then_5() {
        let mut new_game = Game::default();
        new_game.roll(4);
        new_game.roll(5);
        assert_eq!(new_game.score(), 9);
    }

    #[test]
    fn it_should_count_bonus_score_when_a_spare_is_performed() {
        let mut game = Game::default();
        game.roll(7);
        game.roll(3);
        game.roll(5);
        assert_eq!(game.score(), 20);
    }

    #[test]
    fn it_should_count_bonus_score_when_a_spare_is_performed_2() {
        let mut game = Game::default();
        game.roll(7);
        game.roll(3);
        game.roll(5);
        game.roll(2);
        assert_eq!(game.score(), 22);
    }

    #[test]
    fn it_should_count_bonus_score_when_a_strike_is_performed() {
        let mut game = Game::default();
        game.roll(5);
        game.roll(3);
        game.roll(10);
        game.roll(2);
        game.roll(5);
        assert_eq!(game.score(), 32);
    }

    #[test]
    fn it_should_return_a_perfect_score_of_300_with_a_full_game_of_strikes() {
        let mut game = Game::default();
        for i in 0..12 {
            game.roll(10);
        }
        assert_eq!(game.score(), 300);
    }
}
