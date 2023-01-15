#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://user-images.githubusercontent.com/30808373/185181607-1bd61fa3-92dd-4413-b342-7d26861f39b9.png"
)]
//! This is the official rust BetterSchool SDK
//!
//! # Example:
//!
//! ```rust
//! use betterschool_sdk::add;
//!
//! assert_eq!(add(2, 2), 4);
//! ```
//!

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

/// The main container for the sdk
pub struct BetterSchool {
    api_addr: String,
}

/// Represents a school
#[derive(Serialize, Deserialize, Debug)]
pub struct School {
    pub name: String,
    pub schoolID: String,
}

#[derive(Debug)]
pub enum Error {
    ReqwestErr(reqwest::Error),
    SerdeJsonErr(serde_json::Error),
}
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        return Error::ReqwestErr(value);
    }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        return Error::SerdeJsonErr(value);
    }
}

impl BetterSchool {
    /// The main entrypoint for using the sdk
    pub fn new<T: Into<String>>(api_addr: T) -> BetterSchool {
        return BetterSchool {
            api_addr: String::from(api_addr.into()),
        };
    }

    /// Used to get the availible schools
    pub fn get_schools(&self) -> Result<Vec<School>, Error> {
        let body =
            reqwest::blocking::get(self.api_addr.clone() + &String::from("/schools"))?.text()?;

        let parsed: Vec<School> = serde_json::from_str(&body)?;

        return Ok(parsed);
    }
}

/// The main entrypoint for using the sdk
pub fn new<T: Into<String>>(api_addr: T) -> BetterSchool {
    return BetterSchool::new(api_addr);
}
