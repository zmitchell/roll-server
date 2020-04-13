use rand::{thread_rng, distributions::{Distribution, Uniform}};
use crate::parse::{RollCmd, DiceSize};
use std::num::NonZeroU8;

#[derive(Debug, PartialEq)]
pub(crate) struct Roll {
    pub dice: Vec<usize>,
}

pub(crate) fn roll_normal(cmd: &RollCmd) -> Roll {
    let mut rng = thread_rng();
    let distribution = Uniform::new_inclusive(1, cmd.num.get());
    let mut rolls = Vec::new();
    for _ in 0..cmd.num.get() {
        rolls.push(distribution.sample(&mut rng).into());
    }
    Roll {
        dice: rolls,
    }
}

pub(crate) fn roll_crit(cmd: &RollCmd) -> Roll {
    let mut rng = thread_rng();
    let distribution = Uniform::new_inclusive(1, cmd.num.get());
    let mut rolls = Vec::new();
    let num = usize::from(u8::from(cmd.num.get()));
    let size = usize::from(cmd.size);
    let crit = num.checked_mul(size).unwrap();
    rolls.push(crit);
    for _ in 0..cmd.num.get() {
        rolls.push(distribution.sample(&mut rng).into());
    }
    Roll {
        dice: rolls,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normal_rolls_are_nonzero() {
        let cmd = RollCmd {
            num: NonZeroU8::new(4).unwrap(),
            size: DiceSize::D6
        };
        for _ in 0..100 {
            let rolls = roll_normal(&cmd);
            let sum: usize = rolls.dice.iter().sum();
            assert!(sum > 0);
        }
    }

    #[test]
    fn critical_rolls_are_nonzero() {
        let cmd = RollCmd {
            num: NonZeroU8::new(4).unwrap(),
            size: DiceSize::D6
        };
        for _ in 0..100 {
            let rolls = roll_crit(&cmd);
            let sum: usize = rolls.dice.iter().sum();
            assert!(sum > 0);
        }
    }

    #[test]
    fn critical_rolls_higher_than_normal_rolls() {
        let cmd = RollCmd {
            num: NonZeroU8::new(4).unwrap(),
            size: DiceSize::D6
        };
        for _ in 0..100 {
            let normal_rolls = roll_normal(&cmd);
            let crit_rolls = roll_crit(&cmd);
            let normal_sum: usize = normal_rolls.dice.iter().sum();
            let crit_sum: usize = crit_rolls.dice.iter().sum();
            assert!(crit_sum > normal_sum);
        }
    }

    #[test]
    fn critical_rolls_higher_than_max_damage() {
        let cmd = RollCmd {
            num: NonZeroU8::new(4).unwrap(),
            size: DiceSize::D6
        };
        for _ in 0..100 {
            let rolls = roll_crit(&cmd);
            let sum: usize = rolls.dice.iter().sum();
            assert!(sum > 24);
        }
    }
}
