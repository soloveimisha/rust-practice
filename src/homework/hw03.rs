const W: usize = 30; 
const H: usize = 14; 

fn main() {
    let mut output = String::new();

    for y in 0..H {
        for x in 0..W {
            let ch = if y == 0 || y == H - 1 {
                '*'
            } else if x == 0 || x == W - 1 {
                '*'
            } else if x == y * (W - 1) / (H - 1) || x == (W - 1) - y * (W - 1) / (H - 1) {
                '*'
            } else {
                ' '
            };
            output.push(ch);
        }
        output.push('\n');
    }

    print!("{}", output);
}