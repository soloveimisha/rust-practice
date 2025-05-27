use rand::prelude::*;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::rng(); // новая форма вместо thread_rng()
    (0..n).map(|_| rng.random_range(10..100)).collect() // random_range вместо gen_range
}

fn min_adjacent_sum(data: &[i32]) -> Option<((usize, usize), i32)> {
    if data.len() < 2 {
        return None;
    }
    let mut min_sum = data[0] + data[1];
    let mut min_indexes = (0, 1);

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_indexes = (i, i + 1);
        }
    }
    Some((min_indexes, min_sum))
}

fn print_result(data: &[i32], min_pair: ((usize, usize), i32)) {
    let (indexes, sum) = min_pair;
    let (i1, i2) = indexes;

    print!("indexes:");
    for i in 0..data.len() {
        print!(" {:2}.", i);
    }
    println!();

    print!("data:    [");
    for (i, val) in data.iter().enumerate() {
        if i != 0 { print!(", "); }
        print!("{:2}", val);
    }
    println!("]");

    print!("indexes: ");
    for i in 0..data.len() {
        if i == i1 {
            print!("\\__ ");
        } else if i == i2 {
            print!("__/");
        } else {
            print!("    ");
        }
    }
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[i1], data[i2], sum, i1, i2
    );
}

fn main() {
    let data = gen_random_vector(20);
    if let Some(min_pair) = min_adjacent_sum(&data) {
        print_result(&data, min_pair);
    } else {
        println!("Not enough data");
    }
}
