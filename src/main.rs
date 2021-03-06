extern crate rand;
extern crate base64;
extern crate bip39;
extern crate ring_pwhash;
extern crate serde;
extern crate serde_json;
/// TODO: Perhaps, serde_toml
#[macro_use] 
extern crate serde_derive;

use bip39::{ Mnemonic, MnemonicType, Language, Seed };
use rand::Rng;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;

extern crate feather_wallet;

use feather_wallet::{ Salt, LightWallet };

fn main() {
    // let pw = prompt("Please type your password...\n");

    let wallet = LightWallet::default();
    // let priv_key = wallet.master_key();

    let json = serde_json::to_vec(&wallet).unwrap();

    match OpenOptions::new().read(true).write(true).truncate(true).create(true).open("keystore.json") {
        Ok(mut f) => f.write_all(&json),
        _ => panic!("error! error! this should not have occurred"),
    };

    let wallet2 = LightWallet::from_file("keystore.json");

    println!("{:?}\n{:?}", wallet, wallet2);




    // println!("{:?}", priv_key.secret_key);
    // println!("\n{:?}", priv_key);


    // let key: [u8;32] = wallet.derive_pw_key(&pw);

    // println!("{:?}", &key);

    // println!("{}", pw.as_str());
    // let test = Salt::new();

    // println!("{}", test.salt_encoded);
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