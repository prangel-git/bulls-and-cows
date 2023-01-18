use bulls_and_cows::knuth::test_all_secrets;

fn main() {
    let secret_vs_attempts = test_all_secrets();

    for (secret, attempts) in secret_vs_attempts {
        println!("for secret {secret}, knuth uses {attempts} attempts")
    }
}