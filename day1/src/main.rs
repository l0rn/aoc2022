use std::io;

fn main() {
    let mut calorie_sums: Vec<i32> = Vec::new();
    let mut aggregator: i32 = 0;
    for line in io::stdin().lines() {
        let line_string = line.unwrap();
        if line_string == "" {
            calorie_sums.push(aggregator);
            aggregator = 0;
            continue;
        }
        aggregator += line_string.parse::<i32>().unwrap();
    }
    calorie_sums.sort_by(|a, b| b.cmp(a));
    println!(
        "Elf carrying the most calories has {:?} calories",
        calorie_sums.first().unwrap()
    );
    println!(
        "The top three elves carrying the most calories has {:?} calories",
        &calorie_sums[0..3].iter().sum::<i32>()
    );
}
