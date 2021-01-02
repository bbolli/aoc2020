mod aoc;

fn str_count(s: &str, letter: char) -> usize {
    s.chars().filter(|l| *l == letter).count()
}

fn main() {
    let input = aoc::lines_from_file("02.in");
    let mut valid1 = 0;
    let mut valid2 = 0;

    for i in input {
        let parts: Vec<&str> = i.split_whitespace().collect();
        let limits: Vec<usize> = parts[0].split('-').filter_map(|l| l.parse().ok()).collect();
        let lo = limits[0];
        let hi = limits[1];
        let letter = parts[1].chars().nth(0).unwrap();
        let pwd = parts[2];

        let c = str_count(pwd, letter);
        if lo <= c && c <= hi {
            valid1 += 1
        }

        let mut neighbors = String::from("");
        neighbors.push(pwd.chars().nth(lo - 1).unwrap());
        neighbors.push(pwd.chars().nth(hi - 1).unwrap());
        if str_count(&neighbors, letter) == 1 {
            valid2 += 1
        }
    }
    println!("{} {}", valid1, valid2)
}
