use crate::object::Object;
use std::io::Read;

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    IoError(std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeJsonError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

fn get_logged(tag: &str, uri: &str) -> Result<String> {
    let mut response = reqwest::get(uri)?;
    let mut buf = String::new();
    println!(
        "[scryfall] {} '{}' - transferred {} bytes",
        tag,
        uri,
        response.read_to_string(&mut buf)?
    );
    Ok(buf)
}

pub fn get(uri: &str) -> Result<Object> {
    let text = get_logged("get", uri)?;
    Ok(serde_json::from_str(&text)?)
}

pub fn get_bulk(uri: &str) -> Result<Vec<Object>> {
    let text = get_logged("get_bulk", uri)?;
    Ok(serde_json::from_str(&text)?)
}
