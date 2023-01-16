//! The structs used by betterschool_sdk

use serde::{Deserialize, Serialize};

/// Represents a school
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct School {
    /// Human readable name of school
    pub name: String,
    /// The id to be used for further inquireries
    pub schoolID: String,
}

/// Represents a class
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Class {
    /// Human readable name of class
    pub className: String,
    /// The id to be used for further inquireries
    pub classID: String,
}

/// Represents a schedule week
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ScheduleWeek {
    /// Human readable name of class
    pub weekNr: String,
    /// The days of the week
    pub days: Vec<ScheduleDay>,
}

/// Represents a schedule day
#[derive(Serialize, Deserialize, Debug)]
pub struct ScheduleDay {
    /// Name for the day of the week, eg. "Fredag"
    pub name: String,
    /// A string representing the date, eg. "Fri Oct 21 2022"
    pub date: String,

    /// All the individual classes for the day
    pub classes: Vec<ScheduleClass>,
}

/// Represents a singe class
#[derive(Serialize, Deserialize, Debug)]
pub struct ScheduleClass {
    /// The date for the individual class, eg. "21. oktober 2022",
    /// this does not follow the same pattern as the date for the day
    /// as it is scraped directly from visma inschool and not pre parsed.
    pub date: String,
    /// The time for the class, eg. "13:15-14:15", will always follow this structure: "hh:mm-hh:mm"
    pub time: String,
    /// The room for the class, eg. "553"
    pub room: String,
    /// The name of the class, eg. "Naturfag"
    pub name: String,
    /// The name of the teacher for the class
    pub teacher: String,
}

/// The response returned from the add_user method
#[derive(Debug)]
pub struct AddUserResponse {
    /// The status code from the request
    pub code: reqwest::StatusCode,
    /// The response from the API
    pub response: String,
}

impl AddUserResponse {
    /// Used to create a new AddUserResponse from a reqwest blocking response
    pub fn new(response: reqwest::blocking::Response) -> Result<AddUserResponse, reqwest::Error> {
        return Ok(AddUserResponse {
            code: response.status(),
            response: response.text()?,
        });
    }
}
