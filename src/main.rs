use std::{fs::read_to_string, collections::{HashMap, VecDeque}};

fn main() {
    let now = std::time::Instant::now();
    let numbers = read_to_string("./challenge_input.txt").unwrap();
    let numbers: Vec<u128> = numbers.lines().map(|n| n.parse().unwrap()).collect();

    let mut map: HashMap<u128, VecDeque<usize>> = HashMap::new();

    for n in 0..100 {
        match map.get_mut(&numbers[n]) {
            Some(v) => v.push_back(n),
            None => { map.insert(numbers[n], VecDeque::from([n])); },
        }
    }

    let mut mines: Vec<usize> = Vec::new();

    for n in 100..numbers.len() {
        let mut safe = false;

        for p in &numbers[n-100..n] {
            let difference = numbers[n].saturating_sub(*p);

            if difference == 0 {
                continue;
            }

            match map.get(&difference) {
                Some(_) => { safe = true; break; },
                None => (),
            }
        }

        map.get_mut(&numbers[n-100]).unwrap().pop_front();

        match map.get_mut(&numbers[n]) {
            Some(v) => v.push_back(n),
            None => { map.insert(numbers[n], VecDeque::from([n])); },
        }

        if !safe {
            mines.push(n);
        }
    }

    println!("{:?}", mines);
    println!("Done in {} Nanos", now.elapsed().as_nanos());
}
