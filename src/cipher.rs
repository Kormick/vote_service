use agreement;
use byteorder::{ByteOrder, LittleEndian};
use exonum::storage::StorageValue;
use ring::aead;
use schema::{EncryptedVote, Vote};
use std::borrow::Cow;

/// Encrypts vote with service ephemeral key.
pub fn encrypt_vote(vote: &Vote) -> EncryptedVote {
    let key = agreement::get_ephemeral().ephemeral_key;
    let mut enc = CipherChaChaPoly::default();
    enc.set(&key);

    let raw = vote.clone().into_bytes();
    let mut res = [0u8; 128];
    let enc_size = enc.encrypt(0, &[], &raw, &mut res);
    let res = &res[..enc_size];

    let enc_vote = EncryptedVote::new(res.to_vec());

    enc_vote
}

/// Decrypts vote with service ephemeral key.
pub fn decrypt_vote(vote: &EncryptedVote) -> Vote {
    let key = agreement::get_ephemeral().ephemeral_key;
    let mut dec = CipherChaChaPoly::default();
    dec.set(&key);

    let mut dec_output = [0u8; 128];
    let dec_size = dec.decrypt(0, &[], &vote.data(), &mut dec_output).unwrap();
    let dec_output = &dec_output[..dec_size];

    let vote = Vote::from_bytes(Cow::Borrowed(dec_output));

    vote
}

pub const TAGLEN: usize = 16;

/// Trait to implement cipher functionality.
pub trait Cipher: Send + Sync {
    /// The string that the Noise spec defines for the primitive.
    fn name(&self) -> &'static str;

    /// Set the key.
    fn set(&mut self, key: &[u8]);

    /// Encrypt (with associated data) a given plaintext.
    fn encrypt(&self, nonce: u64, authtext: &[u8], plaintext: &[u8], out: &mut [u8]) -> usize;

    #[must_use]
    /// Decrypt (with associated data) a given ciphertext.
    fn decrypt(
        &self,
        nonce: u64,
        authtext: &[u8],
        ciphertext: &[u8],
        out: &mut [u8],
    ) -> Result<usize, ()>;
}

/// CipherChaChaPoly used to store encode/decode keys.
pub struct CipherChaChaPoly {
    sealing: aead::SealingKey,
    opening: aead::OpeningKey,
}

impl Default for CipherChaChaPoly {
    fn default() -> Self {
        Self {
            sealing: aead::SealingKey::new(&aead::CHACHA20_POLY1305, &[0u8; 32]).unwrap(),
            opening: aead::OpeningKey::new(&aead::CHACHA20_POLY1305, &[0u8; 32]).unwrap(),
        }
    }
}

/// Implementation of `Cipher` trait for `CipherChaChaPoly`
impl Cipher for CipherChaChaPoly {
    fn name(&self) -> &'static str {
        "ChaChaPoly"
    }

    /// Set specified key.
    fn set(&mut self, key: &[u8]) {
        self.sealing = aead::SealingKey::new(&aead::CHACHA20_POLY1305, key).unwrap();
        self.opening = aead::OpeningKey::new(&aead::CHACHA20_POLY1305, key).unwrap();
    }

    /// Encrypt data.
    fn encrypt(&self, nonce: u64, authtext: &[u8], plaintext: &[u8], out: &mut [u8]) -> usize {
        let mut nonce_bytes = [0u8; 12];
        LittleEndian::write_u64(&mut nonce_bytes[4..], nonce);

        out[..plaintext.len()].copy_from_slice(plaintext);

        aead::seal_in_place(
            &self.sealing,
            &nonce_bytes,
            authtext,
            &mut out[..plaintext.len() + TAGLEN],
            16,
        ).unwrap();
        plaintext.len() + TAGLEN
    }

    /// Decrypt data.
    fn decrypt(
        &self,
        nonce: u64,
        authtext: &[u8],
        ciphertext: &[u8],
        out: &mut [u8],
    ) -> Result<usize, ()> {
        let mut nonce_bytes = [0u8; 12];
        LittleEndian::write_u64(&mut nonce_bytes[4..], nonce);

        if out.len() >= ciphertext.len() {
            let in_out = &mut out[..ciphertext.len()];
            in_out.copy_from_slice(ciphertext);

            let len = aead::open_in_place(&self.opening, &nonce_bytes, authtext, 0, in_out)
                .map_err(|_| ())?
                .len();

            Ok(len)
        } else {
            let mut in_out = ciphertext.to_vec();

            let out0 = aead::open_in_place(&self.opening, &nonce_bytes, authtext, 0, &mut in_out)
                .map_err(|_| ())?;
            out[..out0.len()].copy_from_slice(out0);
            Ok(out0.len())
        }
    }
}
