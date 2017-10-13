extern crate base64;
extern crate bip39;
extern crate bitcoin;
extern crate rand;
extern crate ring_pwhash;

use bip39::{ Mnemonic, MnemonicType, Language, Seed };
use bitcoin::util::bip32::*;
use rand::Rng;
use ring_pwhash::scrypt::{ ScryptParams, scrypt };

mod network;

use network::Network;

pub struct LightWallet {
    hd_path_string: String,
    seed: Seed,
    salt: Salt,
}

impl Default for LightWallet {
    fn default() -> LightWallet {
        // TODO implement `create(extra_entropy: &[u8])`
        let mnemonic = match Mnemonic::new(MnemonicType::Type12Words, Language::English, "") {
            Ok(b) => b,
            Err(e) => panic!("Error! {}", e)
        };
        println!("{}", mnemonic.get_string()); // Prints out the twelve word phrase
        LightWallet {
            hd_path_string: String::from("m/0'/0'/0'"),
            seed: mnemonic.get_seed(), // Seed struct
            salt: Salt::new(),
        }
    }
}

impl LightWallet {
    pub fn derive_pw_key(&self, pw: &str) -> [u8;32] {
        let log_n: u8 = 14;
        let r: u32 = 8;
        let p: u32 = 1;
        let mut dk_len: [u8;32] = [0u8;32];

        let salt_bytes = base64::decode(&self.salt.salt_encoded).unwrap();
        let scrypt_params = ScryptParams::new(log_n, r, p);
        scrypt(pw.as_bytes(), &salt_bytes, &scrypt_params, &mut dk_len);
        
        dk_len
    }

    pub fn master_key(&self) -> ExtendedPrivKey {
        ExtendedPrivKey::new_master(EC..., Network::Feather, &self.seed).unwrap()
    }
}

pub struct Salt {
    pub salt_encoded: String,
}

impl Salt {
    pub fn new() -> Salt {
        Salt {
            salt_encoded: base64::encode(&Salt::generate_random_32bytes()),
        }
    }

    fn generate_random_32bytes() -> [u8;32] {
        let mut rng = rand::thread_rng();
        let res = rng.gen::<[u8;32]>();
        res
    }
}
