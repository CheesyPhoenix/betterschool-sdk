use betterschool_sdk::BetterSchool;

#[test]
fn full_workflow() {
    // start by creating an instance of BetterSchool with the url to your api, eg. the official "https://api.betterschool.chph.tk"
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

    if schedule.len() == 0 {
        panic!("Got empty schedule");
    }
}
