extern crate rand;
extern crate base64;
extern crate bip39;

use bip39::{Mnemonic, MnemonicType, Language, Seed};
use rand::Rng;

fn main() {
    gen_random_seed();

    println!("{}", generate_salt());
}

fn generate_random_32bytes() -> [u8;32] {
    let mut rng = rand::thread_rng();
    let res = rng.gen::<[u8;32]>();
    res
}

fn generate_salt() -> String {
    base64::encode(&generate_random_32bytes())
}

fn gen_random_seed() {
    let mnemonic_type = MnemonicType::Type12Words;

    let mnemonic = match Mnemonic::new(mnemonic_type, Language::English, "") {
        Ok(b) => b,
        Err(e) => { println!("e: {}", e); return }
    };

    /// get the phrase as a string
    let phrase = mnemonic.get_string();
    println!("phrase: {}", phrase);

    let seed = mnemonic.get_seed();
    let seed_bytes: &[u8] = seed.as_bytes();
    println!("{:?}", seed_bytes);

    // get the HD wallet seed as a hex string
    let seed_hex: &str = seed.as_hex();

    // get an owned Seed instance
    let owned_seed: Seed = seed.to_owned();
}