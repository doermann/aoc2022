mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    assert!(day1::part1() == 68923);
    assert!(day1::part2() == 200044);
    assert!(day2::part1() == 11767);
    assert!(day2::part2() == 13886);
    assert!(day3::part1() == 8153);
    assert!(day3::part2() == 2342);
    assert!(day4::part1() == 475);
    assert!(day4::part2() == 825);
    assert!(day5::part1() == "VQZNJMWTR");
    assert!(day5::part2() == "NLCDCLVMQ");
    assert!(day6::part1() == 1343);
    assert!(day6::part2() == 2193);

    println!("answers: {} {}", day6::part1(), day6::part2());
}
