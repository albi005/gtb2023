use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // 0 = black = .
    // 1 = white = +
    let mut nums = Vec::new();
    let mut num = 0;
    let mut i = 0;
    for c in input.trim().chars() {
        match c {
            '.' => {},
            '+' => num |= 1,
            _ => (),
        }
        if i == 7 {
            nums.push(num);
            num = 0;
            i = 0;
        } else {
            i += 1;
            num <<= 1;
        }
    }

    for num in nums {
        println!("{}", num);
    }
}
