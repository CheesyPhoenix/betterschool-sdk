#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://user-images.githubusercontent.com/30808373/185181607-1bd61fa3-92dd-4413-b342-7d26861f39b9.png"
)]
//! This is the official rust BetterSchool SDK. It is a fully typed wrapper for interacting with the BetterSchool API
//!
//! # Example use:
//!
//! ### Getting data
//! ```rust
//! // import the BetterSchool struct
//! use betterschool_sdk::BetterSchool;
//!
//! // start by creating an instance of BetterSchool with the url to your api,
//! // eg. the official "https://api.betterschool.chph.tk"
//! let better_school = BetterSchool::new("https://api.betterschool.chph.tk");
//!
//! // then get the list of schools for that API
//! let schools = better_school.get_schools().expect("could not get schools");
//!
//! // then select the school you are interested in, get the schoolID
//! // and use it to get the classes for that school
//! let classes = better_school
//!     .get_classes(&schools[0].schoolID)
//!     .expect("Could not get classes");
//!
//! // then select the class you are interested in, get the classID
//! // and use it to get the schedule for that class
//! // (This will return a vector with each element representing a single week)
//! let schedule = better_school
//!     .get_schedule(&schools[0].schoolID, &classes[0].classID)
//!     .expect("Could not get schedule");
//!
//! // then select the week you are interested in, and use it however you like
//! let week = &schedule[0];
//! let week_nr = &week.weekNr;
//! ```
//!
//! ### Adding a new user
//! ```rust, ignore
//! // import the BetterSchool struct
//! use betterschool_sdk::BetterSchool;
//!
//! // start by creating an instance of BetterSchool with the url to your api,
//! // eg. the official "https://api.betterschool.chph.tk"
//! let better_school = BetterSchool::new("https://api.betterschool.chph.tk");
//!
//! // Add a user with the name: "Bob Kåre", password: "Kålmann" and classname: "245A", on the first school returned by get_schools
//! let res = better_school
//!     .add_user(
//!         "Bob Kåre",
//!         "Kålmann",
//!         "245A",
//!         &better_school.get_schools().expect("Could not get schools")[0].schoolID,
//!     )
//!     .expect("Could not add user");
//!
//! // print out the response from the API
//! println!("{:?}", res)
//! ```
//!
//! In this case the response from the API would look like this:
//! ```text
//! AddUserResponse { code: 401, response: "incorrect credentials" }
//! ```
//! Since the credentials aren't valid Feide credentials the API returns a 401 - Unauthorized
//!

#[cfg(test)]
mod tests;

pub mod structure;

use serde_json::json;
use structure::{AddUserResponse, Class, ScheduleWeek, School};

/// The main container for the sdk
pub struct BetterSchool {
    /// The api address to use for api calls
    pub api_addr: String,
    client: reqwest::blocking::Client,
}

/// An error type which implements both the errors from reqwest and serde_json
#[derive(Debug)]
pub enum Error {
    /// A reqwest error
    ReqwestErr(reqwest::Error),
    /// A serde_json error
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
            client: reqwest::blocking::Client::new(),
        };
    }

    /// Used to get the availible schools
    pub fn get_schools(&self) -> Result<Vec<School>, Error> {
        let body = self
            .client
            .get(self.api_addr.clone() + &String::from("/schools"))
            .send()?
            .text()?;

        let parsed: Vec<School> = serde_json::from_str(&body)?;

        return Ok(parsed);
    }

    /// Used to get the availible classes of a school
    pub fn get_classes<T: Into<String>>(&self, school_id: T) -> Result<Vec<Class>, Error> {
        let body = self
            .client
            .get(format!(
                "{}/school/{}/classes",
                self.api_addr,
                school_id.into()
            ))
            .send()?
            .text()?;

        let parsed: Vec<Class> = serde_json::from_str(&body)?;

        return Ok(parsed);
    }

    /// Used to get the schedule for a given class in a give school
    pub fn get_schedule<T: Into<String>>(
        &self,
        school_id: T,
        class_id: T,
    ) -> Result<Vec<ScheduleWeek>, Error> {
        let body = self
            .client
            .get(format!(
                "{}/school/{}/class/{}",
                self.api_addr,
                school_id.into(),
                class_id.into()
            ))
            .send()?
            .text()?;

        let parsed: Vec<ScheduleWeek> = serde_json::from_str(&body)?;

        return Ok(parsed);
    }

    /// Used to add a user/class to the API using your Feide username/password, a class name of your choosing, and the school_id of your school.
    /// This will return a response with the status code and the response from the API. IMPORTANT! if something you entered was incorrect, like the password,
    /// this method will still return an Ok response, so it is important that you check the status code and message to make sure that it works.
    /// (This method also takes a while to complete)
    pub fn add_user<T: Into<String>>(
        &self,
        username: T,
        password: T,
        class_name: T,
        school_id: T,
    ) -> Result<AddUserResponse, Error> {
        let body = serde_json::to_string(&json!({
            "username": username.into(),
            "pass": password.into(),
            "class": class_name.into(),
            "schoolID": school_id.into()
        }))?;

        let res = self
            .client
            .post(format!("{}/addUser", self.api_addr))
            .body(body)
            .send()?;

        return Ok(AddUserResponse::new(res)?);
    }
}
