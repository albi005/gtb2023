fn main() {
    let permutation = "BDFHJLCPRTXVZNYEIWGAKMUSQO";
    let input_letter = 'A';

    let index = input_letter as usize - 'A' as usize;
    let output_letter = permutation.chars().nth(index).unwrap();

    println!("{} -> {}", input_letter, output_letter);

    // Doesn't work with rustc 1.39.0
    // let dict = permutation.chars().zip('A'..).collect::<std::collections::HashMap<_, _>>();
    //
    // let inverse = ('A'..='Z').map(|c| dict[&c]);
    //
    // for i in inverse {
    //     print!("{}", i);
    // }
    // println!();

    let mut inverse = ['A'; 26];
    for (i, c) in permutation.chars().enumerate() {
        inverse[c as usize - 'A' as usize] = (i as u8 + 'A' as u8) as char;
    }
    for i in inverse.iter() {
        print!("{}", i);
    }
}
