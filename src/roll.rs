use rand::{thread_rng, distributions::{Distribution, Uniform}};
use crate::parse::{RollCmd, DiceSize};
use std::num::NonZeroU8;

#[derive(Debug, PartialEq)]
pub(crate) struct Rolls(pub Vec<usize>);

pub(crate) fn roll_normal(cmd: &RollCmd) -> Rolls {
    let mut rng = thread_rng();
    let distribution = Uniform::new_inclusive(1, usize::from(cmd.size));
    let mut rolls = Vec::new();
    for _ in 0..cmd.num.get() {
        rolls.push(distribution.sample(&mut rng).into());
    }
    Rolls(rolls)
}

pub(crate) fn roll_crit(cmd: &RollCmd) -> Rolls {
    let mut rng = thread_rng();
    let distribution = Uniform::new_inclusive(1, usize::from(cmd.size));
    let mut rolls = Vec::new();
    let num = usize::from(u8::from(cmd.num.get()));
    let size = usize::from(cmd.size);
    let crit = num.checked_mul(size).unwrap();
    rolls.push(crit);
    for _ in 0..cmd.num.get() {
        rolls.push(distribution.sample(&mut rng).into());
    }
    Rolls(rolls)
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
            let sum: usize = rolls.0.iter().sum();
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
            let sum: usize = rolls.0.iter().sum();
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
            let normal_sum: usize = normal_rolls.0.iter().sum();
            let crit_sum: usize = crit_rolls.0.iter().sum();
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
            let sum: usize = rolls.0.iter().sum();
            assert!(sum > 24);
        }
    }

    #[test]
    fn no_normal_rolls_higher_than_dice_value() {
        let cmd = RollCmd {
            num: NonZeroU8::new(4).unwrap(),
            size: DiceSize::D6
        };
        let mut dice = Vec::new();
        for _ in 0..100 {
            let rolls = roll_normal(&cmd);
            dice.extend_from_slice(rolls.0.as_slice());
        }
        assert!(*dice.iter().max().unwrap() <= 6)
    }

    #[test]
    fn critical_rolls_less_than_2x_full_damage() {
        let cmd = RollCmd {
            num: NonZeroU8::new(4).unwrap(),
            size: DiceSize::D6
        };
        let mut random_rolls = Vec::new();
        for _ in 0..100 {
            let rolls = roll_crit(&cmd);
            let sum: usize = rolls.0.iter().sum();
            random_rolls.push(sum);
        }
        assert!(*random_rolls.iter().max().unwrap() <= 48)
    }
}
