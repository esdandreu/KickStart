use std::io;

fn solution(kids: &u32, candy_bags: &[u32]) -> u32 {
    candy_bags.iter().sum::<u32>() % kids
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    // 1 <= t <= 100
    let t = buffer.trim().parse::<i8>().unwrap();
    for case in 1..=t {
        // Read first case line
        buffer.clear();
        stdin.read_line(&mut buffer)?;
        let line: Vec<u32> = buffer.trim().split(" ").map(
            |s| s.trim().parse::<u32>().unwrap()
            ).collect();
        let [_n, m] = match line.as_slice() {
            [n, m] => [n, m],
            _ => panic!("Invalid input"),
        };
        // Read second case line
        buffer.clear();
        stdin.read_line(&mut buffer)?;
        let c: Vec<u32> = buffer.trim().split(" ").map(
            |s| s.trim().parse::<u32>().unwrap()
            ).collect();
        // Solve
        let sol = solution(m, &c);
        println!("Case #{}: {}", case, sol)
    }
    Ok(())
}
