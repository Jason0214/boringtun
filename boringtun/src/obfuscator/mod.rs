pub mod quic;

use std::result::Result;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum ObfuscatorType {
    NONE,
    QUIC,
}

impl FromStr for ObfuscatorType {
    type Err = String;

    fn from_str(value: &str) -> Result<ObfuscatorType, String> {
        if value == "none" {
            return Ok(ObfuscatorType::NONE);
        }
        if value == "quic" {
            return Ok(ObfuscatorType::QUIC);
        }

        Err(std::format!("Unknown obfuscator type '{}'!", value))
    }
}
