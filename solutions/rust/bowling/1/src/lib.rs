use std::ops::Index;

use crate::Error::NotEnoughPinsLeft;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq)]
enum State {
    Open,
    Remain,
    Bonus(u32, u32),
    Complete,
}

pub struct BowlingGame {
    history : Vec<u16>,
    state : State,
    frames : u32
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { 
            history: Vec::<u16>::new(),
            state: State::Open,
            frames: 0
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(NotEnoughPinsLeft);
        }
        match self.state {
            State::Open => {
                if pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
                self.history.push(pins);
                if pins == 10 { 
                    self.frames += 1;
                    if self.frames == 10 {
                        self.state = State::Bonus(2, 10);
                    }
                } else { //strike
                    self.state = State::Remain;
                }
            }
            State::Remain => {
                let last = *self.history.clone().last().unwrap();
                if last + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
                self.history.push(pins);
                self.state = State::Open;
                self.frames += 1;
                if last + pins == 10 {
                    if self.frames == 10 {
                        self.state = State::Bonus(1, 10);
                    }
                } else {
                    if self.frames == 10 {
                        self.state = State::Complete;
                        println!();
                    }
                }
            }
            State::Bonus(time, remain) => {
                self.history.push(pins);
                if pins > remain as u16 {
                    return Err(Error::NotEnoughPinsLeft);
                }
                if pins != 10 {
                    self.state = State::Bonus(time, 10 - pins as u32);
                }
                self.frames += 1;
                if self.frames >= 10 + time {
                    self.state = State::Complete;
                }
            }
            State::Complete => return Err(Error::GameComplete)
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.state != State::Complete {
            return None;
        }
        let mut sum = 0;
        let mut index = 0;
        let mut frame = 0;
        while frame < 10 {
            let p1 = self.history.index(index);
            let p2 = self.history.get(index+1).unwrap_or(&0);
            let p3 = self.history.get(index+2).unwrap_or(&0);
            frame += 1;
            if *p1 == 10 { // strike
                sum += p1 + p2 + p3;
                index += 1;
            } else if p1 + p2 == 10 { // spare
                sum += p1 + p2 + p3;
                index += 2;
            } else {
                sum += p1+p2;
                index += 2;
            }
        }
        Some(sum)
    }
}
