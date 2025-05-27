const WIDTH: usize = 11;
const HEIGHT: usize = 11;

fn main() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        let mid = HEIGHT / 2;
        let dist = if y <= mid { y } else { HEIGHT - 1 - y };
        let stars = 2 * dist + 1;
        let spaces = (WIDTH - stars) / 2;

        for _ in 0..spaces {
            output.push(' ');
        }
        for _ in 0..stars {
            output.push('*');
        }
        for _ in 0..spaces {
            output.push(' ');
        }
        output.push('\n');
    }

    print!("{}", output);
}