mod aoc;

fn main() {
    let numbers = aoc::numbers_from_file("01.in");

    'outer1: for a in &numbers {
        for b in &numbers {
            if a + b == 2020 {
                println!("{}", a * b);
                break 'outer1;
            }
        }
    }

    'outer2: for a in &numbers {
        for b in &numbers {
            for c in &numbers {
                if a + b + c == 2020 {
                    println!("{}", a * b * c);
                    break 'outer2;
                }
            }
        }
    }
}
