use std::env;

use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use cached::proc_macro::cached;
use csv::StringRecord;
use hkdf::Hkdf;
use sha2::Sha256;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

#[cached(size = 1)]
fn cipher() -> Aes128Cbc {
    let input_key_material = env::var("PSEUDO_CSV_PASSPHRASE")
        .expect("ERROR: Environment variable PSEUDO_CSV_PASSPHRASE not set!");

    let hkdf = Hkdf::<Sha256>::new(None, input_key_material.as_bytes());

    let mut key = [0u8; 16];
    hkdf.expand(&[], &mut key).unwrap();

    let mut iv = [0u8; 16];
    hkdf.expand(&[], &mut iv).unwrap();

    Aes128Cbc::new_from_slices(&key, &iv).expect("ERROR: Unable to create cipher")
}

#[cached(size = 65_536)]
fn encrypt(plain_text: String) -> String {
    base64::encode(cipher().encrypt_vec(plain_text.as_ref()))
}

pub(crate) trait Pseudo {
    fn pseudonymize(&self, fields_to_encrypt: &[usize]) -> StringRecord;
}

impl Pseudo for StringRecord {
    fn pseudonymize(&self, fields_to_encrypt: &[usize]) -> StringRecord {
        self.iter()
            .enumerate()
            .map(|(ix, field)| match fields_to_encrypt.contains(&ix) {
                true => encrypt(field.to_string()),
                false => field.to_string(),
            })
            .collect()
    }
}
