use std::str::FromStr;

use flutter_rust_bridge::frb;

pub struct Utils {}

impl Utils {
    #[frb(sync)]
    pub fn is_ark(address: &str) -> bool {
        ark_core::ArkAddress::decode(address).is_ok()
    }

    #[frb(sync)]
    pub fn is_btc(address: &str) -> bool {
        bitcoin::Address::from_str(address).is_ok()
    }
}