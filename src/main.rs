use betterschool_sdk::BetterSchool;

fn main() {
    let better_school = BetterSchool::new("https://api.betterschool.chph.tk");

    let schools = better_school.get_schools();

    println!("{:?}", schools);
}
