use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        // YOUR SOLUTION GOES HERE.
        let guess = min + (max - min)/2;
        let x: i32 = player.ask_to_compare(guess);
        if x == 0 {
            return guess;
        }
        else if x == 1 {
            return Self::guess_the_number(player, guess, max);
        }
        else if x == -1 {
            return Self::guess_the_number(player, min, guess);
        }
        else {
            return 0
        }
    }
}
