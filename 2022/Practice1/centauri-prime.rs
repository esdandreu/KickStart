use std::io;

fn solution(kingdom: &str) -> String{
    return match kingdom.trim().to_lowercase().chars().last().unwrap_or('y') {
        'y' => "nobody",
        'a' | 'e' | 'i' | 'o' | 'u' => "Alice",
        _ => "Bob",
    }.to_string();
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    // 1 <= t <= 100
    let t = buffer.trim().parse::<i16>().unwrap_or(0);
    for case in 1..=t {
        // Read first case line
        buffer.clear();
        stdin.read_line(&mut buffer)?;
        // Solve
        let sol = solution(&buffer);
        println!("Case #{}: {} is ruled by {}.", case, buffer.trim(), sol)
    }
    Ok(())
}