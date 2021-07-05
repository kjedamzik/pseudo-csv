use std::env;

use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use cached::proc_macro::cached;
use csv::ByteRecord;
use hkdf::Hkdf;
use sha2::Sha256;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

#[cached(size = 1)]
fn cipher() -> Aes128Cbc {
    let input_key_material = env::var("PSEUDO_CSV_PASSPHRASE")
        .expect("ERROR: Environment variable PSEUDO_CSV_PASSPHRASE not set!");

    let hkdf = Hkdf::<Sha256>::new(None, input_key_material.as_bytes());

    let mut key = [0u8; 16];
    hkdf.expand("pseudo-csv-encryption-key".as_ref(), &mut key)
        .unwrap();

    let mut iv = [0u8; 16];
    hkdf.expand("pseudo-csv-initialization-vector".as_ref(), &mut iv)
        .unwrap();

    Aes128Cbc::new_from_slices(&key, &iv).expect("ERROR: Unable to create cipher")
}

#[cached(size = 65_536)]
fn encrypt(plain_text: Vec<u8>) -> Vec<u8> {
    base64::encode(cipher().encrypt_vec(plain_text.as_ref())).into_bytes()
}

pub(crate) trait Pseudo {
    fn pseudonymize(&self, fields_to_encrypt: &[usize]) -> ByteRecord;
}

impl Pseudo for ByteRecord {
    fn pseudonymize(&self, fields_to_encrypt: &[usize]) -> ByteRecord {
        self.iter()
            .enumerate()
            .map(|(ix, field)| match fields_to_encrypt.contains(&ix) {
                true => encrypt(field.to_vec()),
                false => field.to_vec(),
            })
            .collect()
    }
}
