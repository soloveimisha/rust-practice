fn main() {
    let stars_per_line = [
        1, 1, 3,
        1, 3, 5,
        1, 3, 5, 7,
        1, 3, 5, 7, 9,
        1, 3, 5, 7, 9, 11,
    ];

    let max_stars = 11;

    let mut output = String::new();

    for &stars in &stars_per_line {
        let spaces = (max_stars - stars) / 2;
        output.push_str(&" ".repeat(spaces));
        output.push_str(&"*".repeat(stars));
        output.push('\n');
    }

    print!("{}", output);
}
