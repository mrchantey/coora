use crate::*;
use anyhow::Result;

pub const CREDENTIALS_MAX_LEN: usize = 50;
// pub ssid: [u8; CREDENTIALS_MAX_LEN],
// pub ssid_len: usize,
// pub pass: [u8; CREDENTIALS_MAX_LEN],
// pub pass_len: usize,
const STORE_SSID: &str = "ssid";
const STORE_PASS: &str = "pass";

pub struct WifiCredentials {
    pub ssid: String,
    pub pass: String,
}

impl WifiCredentials {
    pub fn set(store: &Store, ssid: &str, pass: &str) -> Result<()> {
        store.set(STORE_SSID, ssid.as_bytes())?;
        store.set(STORE_PASS, pass.as_bytes())?;
        Ok(())
    }
    pub fn set_ssid(store: &Store, ssid: &[u8]) -> Result<()> {
        store.set(STORE_SSID, ssid)?;
        Ok(())
    }
    pub fn set_pass(store: &Store, pass: &[u8]) -> Result<()> {
        store.set(STORE_PASS, pass)?;
        Ok(())
    }
    pub fn clear(store: &Store) -> Result<()> {
        store.remove(STORE_SSID)?;
        store.remove(STORE_PASS)?;
        // store.g
        Ok(())
    }

    pub fn get(store: &Store) -> Result<WifiCredentials> {
        let ssid = store.get_string::<{ CREDENTIALS_MAX_LEN }>(STORE_SSID)?;
        let pass = store.get_string::<{ CREDENTIALS_MAX_LEN }>(STORE_PASS)?;
        Ok(WifiCredentials { ssid, pass })
    }
}
