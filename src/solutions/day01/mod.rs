use std::io::Read;

pub fn part1() -> anyhow::Result<()> {
    let mut file = std::fs::File::open("./src/solutions/day01/input.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in contents.lines() {
        let mut parts = line
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().unwrap());
        let (fst, snd) = (parts.next().unwrap(), parts.next().unwrap());
        vec1.push(fst);
        vec2.push(snd);
    }

    vec1.sort();
    vec2.sort();

    let res: i64 = vec1.iter().zip(vec2).map(|(a, b)| (a-b).abs()).sum();

    println!("{}", res);

    Ok(())
}
