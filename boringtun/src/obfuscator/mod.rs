use std::str::FromStr;
use std::result::Result;

#[derive(Copy, Clone, Debug)]
pub enum ObfuscatorType {
    QUIC,
}

impl FromStr for ObfuscatorType {
    type Err = String;

    fn from_str(value: &str) -> Result<ObfuscatorType, String> {
        if value == "quic" {
            return Ok(ObfuscatorType::QUIC);
        }

        Err(std::format!("Unknown obfuscator type '{}'!", value))
    }
}

