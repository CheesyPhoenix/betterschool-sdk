# BetterSchool SDK

This is the official rust BetterSchool SDK. It is a fully typed wrapper for interacting with the BetterSchool API

## Install:

```toml
[dependecies]
betterschool-sdk = "1.0.1"
```

or

```
cargo add betterschool-sdk
```

<hr>

## Example use:

### Getting data

```rust
// import the BetterSchool struct
use betterschool_sdk::BetterSchool;

// start by creating an instance of BetterSchool with the url to your api,
// eg. the official "https://api.betterschool.chph.tk"
let better_school = BetterSchool::new("https://api.betterschool.chph.tk");

// then get the list of schools for that API
let schools = better_school.get_schools().expect("could not get schools");

// then select the school you are interested in, get the schoolID
// and use it to get the classes for that school
let classes = better_school
    .get_classes(&schools[0].schoolID)
    .expect("Could not get classes");

// then select the class you are interested in, get the classID
// and use it to get the schedule for that class
// (This will return a vector with each element representing a single week)
let schedule = better_school
    .get_schedule(&schools[0].schoolID, &classes[0].classID)
    .expect("Could not get schedule");

// then select the week you are interested in, and use it however you like
let week = &schedule[0];
let week_nr = &week.weekNr;
```

### Adding a new user

```rust
// import the BetterSchool struct
use betterschool_sdk::BetterSchool;

// start by creating an instance of BetterSchool with the url to your api,
// eg. the official "https://api.betterschool.chph.tk"
let better_school = BetterSchool::new("https://api.betterschool.chph.tk");

// Add a user with the name: "Bob Kåre", password: "Kålmann" and classname: "245A", on the first school returned by get_schools
let res = better_school
    .add_user(
        "Bob Kåre",
        "Kålmann",
        "245A",
        &better_school.get_schools().expect("Could not get schools")[0].schoolID,
    )
    .expect("Could not add user");

// print out the response from the API
println!("{:?}", res)
```

In this case the response from the API would look like this:

```json
AddUserResponse { code: 401, response: "incorrect credentials" }
```

Since the credentials aren't valid Feide credentials the API returns a 401 - Unauthorized
