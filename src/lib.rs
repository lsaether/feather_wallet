extern crate base64;
extern crate bip39;
extern crate rand;

use bip39::{Mnemonic, MnemonicType, Language, Seed};
use rand::Rng;

pub struct LightWallet {
    hd_path_string: String,
    seed: Seed,
    salt: Salt,
}

impl Default for LightWallet {
    pub fn default() -> LightWallet {
        let mnemonic = match Mnemonic::new(MnemonicType::Type12Words, Language::English, "") {
            Ok(b) => b,
            Err(e) => panic!("Error! {}", e)
        };
        LightWallet {
            hd_path_string: String::from("m/0'/0'/0"),
            seed: mnemonic.get_seed(),
            salt: Salt::new(),
        }
    }
}

impl LightWallet {
    pub fn deriveKey(&self, pw: &str) -> [u8] {
        let logN = 14;
        let r = 8;
        let dkLen = 32;
        let interruptStep = 200;
    }
}

pub struct Salt {
    pub salt: String,
}

impl Salt {
    pub fn new() -> Salt {
        Salt {
            salt: base64::encode(&Salt::generate_random_32bytes()),
        }
    }

    fn generate_random_32bytes() -> [u8;32] {
        let mut rng = rand::thread_rng();
        let res = rng.gen::<[u8;32]>();
        res
    }
}
