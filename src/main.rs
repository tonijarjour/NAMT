use std::sync::{Arc, Mutex};
use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

fn main() {
    let now = std::time::Instant::now();

    let numbers = read_to_string("./challenge_input.txt").unwrap();
    let numbers: Vec<u128> = numbers.lines().map(|n| n.parse().unwrap()).collect();
    let num_len = numbers.len();

    let numbers = Arc::new(numbers);
    let res = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    let args: Vec<String> = std::env::args().collect();
    let threads: usize = if args.len() == 2 {
        args[1].parse().unwrap()
    } else {
        2
    };

    for n in 0..threads {
        let res = Arc::clone(&res);
        let numbers = Arc::clone(&numbers);

        let handle = std::thread::spawn(move || {
            let mut crumbles = verify_range(
                &numbers,
                (num_len * n / threads) + 100,
                num_len * (n + 1) / threads,
            );

            let mut vec = res.lock().unwrap();
            vec.append(&mut crumbles);
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    let res = res.lock().unwrap();
    println!("Answer: {}", numbers[res[0]]);
    println!("Done in {} Nanos", now.elapsed().as_nanos());
}

fn verify_range(numbers: &Arc<Vec<u128>>, start: usize, end: usize) -> Vec<usize> {
    let mut map: HashMap<u128, VecDeque<usize>> = HashMap::new();

    for (i, n) in numbers[start - 100..].iter().enumerate().take(100) {
        match map.get_mut(n) {
            Some(v) => v.push_back(i),
            None => {
                map.insert(*n, VecDeque::from([i]));
            }
        }
    }

    let mut mines: Vec<usize> = Vec::new();

    for n in start..end {
        let mut safe = false;

        for p in &numbers[n - 100..n] {
            let difference = numbers[n].saturating_sub(*p);

            if difference == 0 {
                continue;
            }

            if let Some(v) = map.get(&difference) {
                if !v.is_empty() {
                    safe = true;
                    break;
                }
            }
        }

        map.get_mut(&numbers[n - 100]).unwrap().pop_front();
        match map.get_mut(&numbers[n]) {
            Some(v) => v.push_back(n),
            None => {
                map.insert(numbers[n], VecDeque::from([n]));
            }
        }

        if !safe {
            mines.push(n);
        }
    }

    mines
}
