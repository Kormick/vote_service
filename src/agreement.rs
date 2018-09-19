use ring::{agreement, error, rand};
use std::sync::Mutex;
use untrusted;

pub struct KeyPair {
    pub secret: agreement::EphemeralPrivateKey,
    pub public: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct EphemeralKeys {
    pub public_out_key: Vec<u8>,
    pub ephemeral_key: Vec<u8>,
}

lazy_static! {
    static ref EPHEMERAL_KEYS: Mutex<EphemeralKeys> = Mutex::new(EphemeralKeys {
        public_out_key: vec![],
        ephemeral_key: vec![],
    });
}

pub fn init_ephemeral(peer_public: &[u8]) {
    let key_pair = generate_key_pair();
    let ephemeral = generate_ephemeral(peer_public, key_pair.secret);

    EPHEMERAL_KEYS.lock().unwrap().public_out_key = ephemeral.public_out_key;
    EPHEMERAL_KEYS.lock().unwrap().ephemeral_key = ephemeral.ephemeral_key;
}

pub fn get_ephemeral() -> EphemeralKeys {
    let keys = EPHEMERAL_KEYS.lock().unwrap();
    EphemeralKeys {
        public_out_key: keys.public_out_key.clone(),
        ephemeral_key: keys.ephemeral_key.clone(),
    }
}

fn generate_key_pair() -> KeyPair {
    let rng = rand::SystemRandom::new();
    let private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();

    let mut public_key = [0u8; agreement::PUBLIC_KEY_MAX_LEN];
    let public_key = &mut public_key[..private_key.public_key_len()];
    let res = private_key.compute_public_key(public_key);
    if res.is_err() {
        panic!("agreement::generate_key_pair: failed to compute public key");
    }

    KeyPair {
        secret: private_key,
        public: public_key.to_vec(),
    }
}

fn generate_ephemeral(
    peer_public: &[u8],
    secret_key: agreement::EphemeralPrivateKey,
) -> EphemeralKeys {
    let mut public_key = [0u8; agreement::PUBLIC_KEY_MAX_LEN];
    let public_key = &mut public_key[..secret_key.public_key_len()];
    let res = secret_key.compute_public_key(public_key);
    if res.is_err() {
        panic!("agreement::generate_ephemeral: failed to compute public key");
    }

    let peer_key = untrusted::Input::from(peer_public);
    let key_alg = &agreement::X25519;

    let res =
        agreement::agree_ephemeral(secret_key, key_alg, peer_key, error::Unspecified, |kdf| {
            Ok(kdf.to_vec())
        }).unwrap();

    EphemeralKeys {
        public_out_key: public_key.to_vec(),
        ephemeral_key: res,
    }
}
