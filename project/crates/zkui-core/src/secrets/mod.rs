//! 秘密存储：代理密码等写入系统凭据存储，不落配置文件。

use crate::error::{AppResult, app};

use crate::config::constants::{KEYRING_SERVICE, PROXY_ACCOUNT_PREFIX};

fn entry(account: &str) -> Result<keyring::Entry, keyring::Error> {
    keyring::Entry::new(KEYRING_SERVICE, account)
}

/// 代理密码的 key 前缀。
fn proxy_account(profile: &str) -> String {
    format!("{PROXY_ACCOUNT_PREFIX}{profile}")
}

pub fn set_proxy_password(profile: &str, password: &str) -> AppResult<()> {
    entry(&proxy_account(profile))
        .and_then(|entry| entry.set_password(password))
        .map_err(|error| Box::new(app::secret_store_failed(&error.to_string())))
}

pub fn get_proxy_password(profile: &str) -> AppResult<Option<String>> {
    match entry(&proxy_account(profile)).and_then(|entry| entry.get_password()) {
        Ok(password) => Ok(Some(password)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(error) => Err(Box::new(app::secret_store_failed(&error.to_string()))),
    }
}

pub fn delete_proxy_password(profile: &str) -> AppResult<()> {
    match entry(&proxy_account(profile)).and_then(|entry| entry.delete_credential()) {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(error) => Err(Box::new(app::secret_store_failed(&error.to_string()))),
    }
}
