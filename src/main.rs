extern crate rand;
extern crate base64;
extern crate bip39;
extern crate ring_pwhash;

use bip39::{Mnemonic, MnemonicType, Language, Seed};
use rand::Rng;
use std::io;
use std::io::prelude::*;

extern crate feather_wallet;

use feather_wallet::{ Salt, LightWallet };

fn main() {
    let pw = prompt("Please type your password...\n");

    let wallet = LightWallet::default();
    let key: [u8;32] = wallet.derive_key(&pw);

    println!("{:?}", &key);

    println!("{}", pw.as_str());
    let test = Salt::new();

    println!("{}", test.salt_encoded);
}

fn prompt(message: &str) -> String {
    println!("{}", &message);

    let mut input = String::new();

    io::stdin().read_line(&mut input);
    let res = String::from(input.trim());
    res
}


// fn gen_random_seed() {
//     let mnemonic_type = MnemonicType::Type12Words;

//     let mnemonic = match Mnemonic::new(mnemonic_type, Language::English, "") {
//         Ok(b) => b,
//         Err(e) => { println!("e: {}", e); return }
//     };

//     /// get the phrase as a string
//     let phrase = mnemonic.get_string();
//     println!("phrase: {}", phrase);

//     let seed = mnemonic.get_seed();
//     let seed_bytes: &[u8] = seed.as_bytes();
//     println!("{:?}", seed_bytes);

//     // get the HD wallet seed as a hex string
//     let seed_hex: &str = seed.as_hex();

//     // get an owned Seed instance
//     let owned_seed: Seed = seed.to_owned();
// }