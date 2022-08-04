use std::collections::HashMap;
use std::thread;

// RESULTS
// test bench_large_parallel   ... bench:     114,105 ns/iter (+/- 1,816)
// test bench_large_sequential ... bench:     397,330 ns/iter (+/- 3,721)
// test bench_small_parallel   ... bench:      47,283 ns/iter (+/- 585)
// test bench_small_sequential ... bench:      14,012 ns/iter (+/- 1,005)
// test bench_tiny_parallel    ... bench:      15,059 ns/iter (+/- 299)
// test bench_tiny_sequential  ... bench:          44 ns/iter (+/- 0)

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut thread_pool = vec![];
    let mut letters: HashMap<char, usize> = HashMap::new();

    for lines in input.chunks(input.len() / worker_count + 1) {
        let string = lines.join("");
        let worker = thread::spawn(move || {
            let mut letters: HashMap<char, usize> = HashMap::new();
            for c in string
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| c.to_ascii_lowercase()) {
                if c .is_alphabetic() {
                    *letters.entry(c).or_default() += 1;
                }
            }
            letters
        });
        thread_pool.push(worker);
    }

    for thread in thread_pool {
        let response = thread.join().unwrap();
        for (key, value) in response {
            *letters.entry(key).or_default() += value;
        }
    }

    letters
}
