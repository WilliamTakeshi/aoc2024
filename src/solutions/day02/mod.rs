pub fn part1() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./src/solutions/day02/input.txt")?;

    let count = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<i64>)
                .collect::<Result<Vec<_>, _>>()
        })
        .filter_map(Result::ok) // skip lines that failed to parse
        .filter(|nums| is_safe(nums))
        .count();

    println!("{}", count);
    Ok(())
}

fn is_safe(input: &Vec<i64>) -> bool {
    let zipped = input.iter().zip(input.iter().skip(1));

    let diff = zipped.map(|(a, b)| a - b).collect::<Vec<_>>();

    diff.iter().all(|&v| v == 1 || v == 2 || v == 3)
        || diff.iter().all(|&v| v == -1 || v == -2 || v == -3)
}
