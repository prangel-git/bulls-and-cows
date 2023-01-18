use crate::{CodeWord, utils::codeword_to_codedigits};

use super::play_against_knuth;
type NumAttempts = u8;

const CODEMAX: u16 = 1000;

pub fn test_all_secrets() -> Vec<(CodeWord, NumAttempts)> {
    let mut secret_vs_attempts = Vec::new();

    for codeword in 0..CODEMAX {
        let secret = codeword_to_codedigits(codeword);
        let log = play_against_knuth(&secret);
        secret_vs_attempts.push((codeword, log.len() as NumAttempts));
    }

    return secret_vs_attempts;
}