const SERVICE_NAME: &str = "com.ingine.torn-nexus";
use keyring_core::{Entry, Error};

pub fn get_key(account: &str) -> Result<Option<String>, Error> {
    let entry = Entry::new(SERVICE_NAME, account)?;
    match entry.get_password() {
        Ok(key) => Ok(Some(key)),
        Err(err) => match err {
            Error::NoEntry => Ok(None),
            _ => Err(err),
        },
    }
}

pub fn set_key(account: &str, key: &str) -> Result<(), Error> {
    let entry = Entry::new(SERVICE_NAME, account)?;
    entry.set_password(key)
}

pub fn delete_key(account: &str) -> Result<(), Error> {
    let entry = Entry::new(SERVICE_NAME, account)?;
    entry.delete_credential()
}
