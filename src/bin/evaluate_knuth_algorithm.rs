use bulls_and_cows::knuth::play_against_knuth;

fn main() {
    let secret = [1, 2, 3, 4];
    let log = play_against_knuth(&secret);

    for (guess, clue) in &log {
        println!("Knut's guess {:?}, clue {:?}", guess, clue);
    }

    println!("number of attempts {}", log.len());
}