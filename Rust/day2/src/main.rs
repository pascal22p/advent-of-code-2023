use day2::day2_part1;


fn main() {
    let result1 = day2_part1("test_input");
    let sum1: i32 = result1.iter().sum();
    print!("{}", "The result for part 1 test is: ");
    println!("{}", sum1);

    let result1 = day2_part1("input");
    let sum1: i32 = result1.iter().sum();
    print!("{}", "The result for part 1 is: ");
    println!("{}", sum1);

    /* 
    let result2 = day1_part2("input");
    let sum2: i32 = result2.iter().sum();
    print!("{}", "The result for part 2 is: ");
    println!("{}", sum2);
    */
}