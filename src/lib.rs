extern crate base64;
extern crate bip39;
extern crate rand;

use bip39::{Mnemonic, MnemonicType, Language, Seed};
use rand::Rng;

struct LightWallet {
    hd_path_string: String,
    seed: Seed,
    salt: Salt,
}

impl Default for LightWallet {
    fn default() -> LightWallet {
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

struct Salt {
    salt: String,
}

impl Salt {
    pub fn new() -> Salt {
        Salt {
            salt: base64::encode(&Salt::generate_random_32bytes()),
        }
    }

    pub fn generate_random_32bytes() -> [u8;32] {
        let mut rng = rand::thread_rng();
        let res = rng.gen::<[u8;32]>();
        res
    }
}
