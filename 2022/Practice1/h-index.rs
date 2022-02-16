use std::io;
use std::cmp::min;
use std::collections::BTreeMap;

// It is too slow
fn solution(n: u32, c: &mut [u32]) -> u32 {
    // n number of papers = length of c
    // c[i] = number of citations of paper i
    let mut h: u32 = 0;
    let mut h_map = BTreeMap::new();
    // papers[i] = counts of papers that have i citations
    for i in 0..n {
        // Truncate citations to the maximum number
        let c_i: u32 = min(c[i as usize], n);
        // If is relevant, add it to the h_map
        if c_i > h {
            // Update the h_map
            *h_map.entry(c_i).or_insert(0) += 1;
            // Update the h-index
            h = 0;
            for (k, v) in h_map.iter().rev() {
                if k > &h {
                    h = min(h + v, *k);
                } else {
                    break;
                }
            }
            // Filter out unused entries (`retain` is a new feature)
            for k in h_map.clone().keys() {
                if k < &h {
                    h_map.remove(k);
                }
            }
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
        let n = buffer.trim().parse::<u32>().unwrap();
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