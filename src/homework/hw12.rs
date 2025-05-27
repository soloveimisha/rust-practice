use rand::prelude::*;

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::rng();
    (0..n).map(|_| rng.random_range(1..=10)).collect()
}

fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return None; 
    }

    let average = total / n;
    let mut moves = 0;
    let mut balance = 0;

    for &load in shipments {
        balance += load as i32 - average as i32;
        moves += balance.abs() as usize;
    }

    Some(moves)
}


fn print_example(shipments: Vec<u32>) {
    println!("Вантажі: {:?}", shipments);
    match count_permutation(&shipments) {
        Some(moves) => println!("Мінімальна кількість переміщень: {}\n", moves),
        None => println!("Неможливо рівномірно розподілити вантаж\n"),
    }
}

fn main() {
    // Приклад 1
    let shipments1 = vec![8, 2, 2, 4, 4];
    print_example(shipments1);

    // Приклад 2
    let shipments2 = vec![9, 3, 7, 2, 9];
    print_example(shipments2);

    // Генерація випадкового прикладу
    let shipments3 = gen_shipments(5);
    print_example(shipments3);
}
