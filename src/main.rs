use betterschool_sdk::BetterSchool;

fn main() {
    let better_school = BetterSchool::new("https://api.betterschool.chph.tk");

    let schools = better_school.get_schools().expect("could not get schools");

    let classes = better_school
        .get_classes(&schools[0].schoolID)
        .expect("Could not get classes");

    let schedule = better_school
        .get_schedule(&schools[0].schoolID, &classes[0].classID)
        .expect("Could not get schedule");

    println!("{:?}", schedule);
}
