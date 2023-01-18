
use crate::{CodeDigits, Clue};
use crate::utils::find_bulls_and_cows;
use super::mindstate::MindState;


pub fn play_against_knuth(secret: &[u8; 4]) -> Vec<(CodeDigits, Clue)> {
    let mut log = Vec::new();

    let mut mindstate = MindState::new();

    let mut current_guess = mindstate.get_guess();
    let mut current_clue =  find_bulls_and_cows(&secret, &current_guess);

    log.push((current_guess.clone(), current_clue.clone()));

    while secret != &current_guess {
        mindstate.update_mindstate(&current_guess, &current_clue);

        current_guess = mindstate.get_guess();
        current_clue =  find_bulls_and_cows(&secret, &current_guess);

        log.push((current_guess.clone(), current_clue.clone()));      
    }

    return log;
}