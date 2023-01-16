use super::*;

#[test]
fn try_get_schools() {
    let better_school = BetterSchool::new("https://api.betterschool.chph.tk");

    let schools = better_school.get_schools().expect("could not get schools");

    if schools.len() == 0 {
        panic!("Got empty school list");
    }
}

#[test]
fn try_get_classes() {
    let better_school = BetterSchool::new("https://api.betterschool.chph.tk");

    let schools = better_school.get_schools().expect("could not get schools");

    let classes = better_school
        .get_classes(&schools[0].schoolID)
        .expect("Could not get classes");

    if classes.len() == 0 {
        panic!("Got empty class list for school: {}", schools[0].name);
    }
}

#[test]
fn try_get_schedule() {
    let better_school = BetterSchool::new("https://api.betterschool.chph.tk");

    let schools = better_school.get_schools().expect("could not get schools");

    let classes = better_school
        .get_classes(&schools[0].schoolID)
        .expect("Could not get classes");

    let schedule = better_school
        .get_schedule(&schools[0].schoolID, &classes[0].classID)
        .expect("Could not get schedule");

    if schedule.len() == 0 {
        panic!("Got empty schedule");
    }
}

#[test]
#[ignore = "expensive"]
fn try_add_user() {
    let better_school = BetterSchool::new("https://api.betterschool.chph.tk");

    let res = better_school
        .add_user(
            "Bob Kåre",
            "Kålmann",
            "245A",
            &better_school.get_schools().expect("Could not get schools")[0].schoolID,
        )
        .expect("Could not add user");

    assert_eq!(res.code.as_u16(), 401);
    assert_eq!(res.response, "incorrect credentials");
}
