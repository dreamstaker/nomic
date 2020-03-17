use std::fs;
use std::path::Path;
use log::info;
use nomic_primitives::Result;
use nomic_bitcoin::bitcoin;
use nomic_signatory_set::SignatorySet;

pub struct Wallet {
    privkey: secp256k1::SecretKey
}

impl Wallet {
    pub fn load_or_generate<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        let privkey_bytes = if path.exists() {
            info!("Loading existing wallet from {:?}", path);
            fs::read(path)?
        } else {
            info!("Generating new wallet at {:?}", path);
            let bytes: [u8; 32] = rand::random();
            fs::write(path, bytes)?;
            bytes.to_vec()
        };

        let privkey = secp256k1::SecretKey::from_slice(
            privkey_bytes.as_slice()
        )?;

        Ok(Wallet { privkey })
    }

    pub fn pubkey(&self) -> secp256k1::PublicKey {
        let secp = secp256k1::Secp256k1::signing_only();
        secp256k1::PublicKey::from_secret_key(&secp, &self.privkey)
    }

    pub fn deposit_address(&self, signatories: &SignatorySet) -> bitcoin::Address {
        let pubkey_bytes = bitcoin::PublicKey {
            compressed: true,
            key: self.pubkey()
        }.to_bytes();
        let script = nomic_signatory_set::output_script(
            signatories,
            pubkey_bytes
        );
        bitcoin::Address::p2wsh(&script, bitcoin::Network::Testnet)
    }
}

