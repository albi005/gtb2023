fn main() {
    let iii = "BDFHJLCPRTXVZNYEIWGAKMUSQO";
    let ii = "AJDKSIRUXBLHWTMCQGZNPYFVOE";
    let i = "EKMFLGDQVZNTOWYHXUSPAIBRCJ";
    let ukw = "YRUHQSLDPXNGOKMIEBFZCWVJAT";
    let message = "UPUVESZZIBEOICCZXLTWEIPHUMJLWJDTHBSPVCWMPQRJKZFBKVVOCX";

    let mut iii = Rotor::new(iii);
    let mut ii = Rotor::new(ii);
    let mut i = Rotor::new(i);
    let ukw = Rotor::new(ukw);

    for c in message.chars() {
        let mut c = c;
        c = iii.forward(c);
        c = ii.forward(c);
        c = i.forward(c);
        c = ukw.forward(c);
        c = i.backward(c);
        c = ii.backward(c);
        c = iii.backward(c);
        print!("{}", c);
        if iii.rotate() {
            if ii.rotate() {
                i.rotate();
            }
        }
    }
    println!();
}

struct Rotor {
    permutation: [char; 26],
    inverse: [char; 26],
    position: usize,
}

impl Rotor {
    fn new(permutation: &str) -> Self {
        let mut inverse = ['A'; 26];
        for (i, c) in permutation.chars().enumerate() {
            inverse[c as usize - 'A' as usize] = (i as u8 + 'A' as u8) as char;
        }
        Self {
            permutation: permutation.chars().collect::<Vec<_>>().try_into().unwrap(),
            inverse,
            position: 0,
        }
    }

    fn forward(&self, input: char) -> char {
        let mut index = input as usize - 'A' as usize;
        index = (index + self.position) % 26;
        self.permutation[index]
    }

    fn backward(&self, input: char) -> char {
        let mut rotated_permutation = ['A'; 26];
        for i in 0..26 {
            rotated_permutation[i] = self.permutation[(i + self.position) % 26];
        }

        let mut inverse = ['A'; 26];
        for (i, c) in rotated_permutation.iter().enumerate() {
            inverse[*c as usize - 'A' as usize] = (i as u8 + 'A' as u8) as char;
        }

        let index = input as usize - 'A' as usize;
        inverse[index]
    }

    fn rotate(&mut self) -> bool {
        self.position = (self.position + 1) % 26;
        self.position == 0
    }
}
