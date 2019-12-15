#[macro_use]
extern crate log;

use std::panic;
use wasm_bindgen::prelude::*;
use cfg_if::cfg_if;

use nimiq_keys::{Address, AddressParseError, PrivateKey, PublicKey};
use nimiq_utils::key_rng::SecureGenerate;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn init() -> Result<(), JsValue> {
    let config = wasm_logger::Config::new(log::Level::Debug);
    wasm_logger::init(config);

    panic::set_hook(Box::new(console_error_panic_hook::hook));

    info!("WASM playground initialized");

    Ok(())
}

#[wasm_bindgen]
pub struct GeneratedAddress {
    private_key: String,
    address: String,
}

#[wasm_bindgen]
impl GeneratedAddress {
    pub fn generate() -> GeneratedAddress {
        let private_key = PrivateKey::generate_default_csprng();
        let public_key = PublicKey::from(&private_key);
        let address = Address::from(&public_key);
        GeneratedAddress {
            private_key: private_key.to_hex(),
            address: address.to_user_friendly_address()
        }
    }

    pub fn private_key(&self) -> String {
        self.private_key.clone()
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }
}
