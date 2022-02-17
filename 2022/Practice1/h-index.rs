use std::io;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// It is too slow
fn solution(n: usize, c: &mut [u32]) -> u32 {
    // n number of papers = length of c
    // c[i] = number of citations of paper i
    let mut h: u32 = 0;
    // A sorted list of the papers with the most citations. In order to be able
    // to peek to the lowest value, we must use `Reverse` when pushing values
    // into the list.
    let mut papers = BinaryHeap::<Reverse<u32>>::with_capacity(n);
    for i in 0..n {
        // Remove papers from the list if they are not relevant for our current
        // h-index. If the queue is empty we will not remove anything.
        while papers.peek().unwrap_or(&Reverse(n as u32)) >= &Reverse(h) {
            papers.pop();
        }
        // Push a paper relevant to our h-index into our queue
        if c[i] > h {
            papers.push(Reverse(c[i]));
        }
        // Increase the h-index if we have more papers with citations than our
        // current h-index
        if (papers.len() as u32) == h+1 {
            h += 1;
        }
        // No need to spend memory keeping track of the results
        // print!("{:?} ", h_map);
        print!("{} ", h);
    }
    return h;
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    // 1 <= t <= 100
    let t = buffer.trim().parse::<i8>().unwrap_or(0);
    for case in 1..=t {
        // Read first case line
        buffer.clear();
        stdin.read_line(&mut buffer)?;
        let n = buffer.trim().parse::<usize>().unwrap();
        // Read second case line
        buffer.clear();
        stdin.read_line(&mut buffer)?;
        let mut c: Vec<u32> = buffer.trim().split(" ").map(
            |s| s.trim().parse::<u32>().unwrap()
            ).collect();
        // Solve
        print!("Case #{}: ", case);
        solution(n, &mut c);
        println!("");
    }
    Ok(())
}