use std::collections::HashMap;
use std::io::Read;

pub fn part1() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./src/solutions/day01/input.txt")?;

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in contents.lines() {
        let mut parts = line.split_whitespace().map(|s| s.parse::<i64>().unwrap());
        let (fst, snd) = (parts.next().unwrap(), parts.next().unwrap());
        vec1.push(fst);
        vec2.push(snd);
    }

    vec1.sort();
    vec2.sort();

    let res: i64 = vec1.iter().zip(vec2).map(|(a, b)| (a - b).abs()).sum();

    println!("{}", res);

    Ok(())
}

pub fn part2() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./src/solutions/day01/input.txt")?;

    let mut vec1 = Vec::new();
    let mut hm2 = HashMap::new();

    for line in contents.lines() {
        let mut parts = line.split_whitespace().map(|s| s.parse::<i64>().unwrap());
        let (fst, snd) = (parts.next().unwrap(), parts.next().unwrap());
        vec1.push(fst);
        hm2.entry(snd).and_modify(|snd| *snd += 1).or_insert(1);
    }

    let res: i64 = vec1.iter().map(|&a| a * hm2.get(&a).unwrap_or(&0)).sum();

    println!("{}", res);

    Ok(())
}
