use betterschool_sdk::BetterSchool;

fn main() {
    let better_school = BetterSchool::new("https://api.betterschool.chph.tk");

    let res = better_school
        .add_user(
            "Bob Kåre",
            "Kålmann",
            "245A",
            &better_school.get_schools().expect("Could not get schools")[0].schoolID,
        )
        .expect("Could not add user");

    println!("{:?}", res)
}
