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

fn to_char(i: usize) -> char {
    (i + 'A' as usize) as u8 as char
}

fn to_index(c: char) -> usize {
    c as usize - 'A' as usize
}

struct Rotor {
    permutation: [char; 26],
    position: usize,
}

impl Rotor {
    fn new(permutation: &str) -> Self {
        let mut rotor = Self {
            permutation: ['A'; 26],
            position: 0,
        };
        for (i, c) in permutation.chars().enumerate() {
            rotor.permutation[i] = c;
        }
        rotor
    }

    fn forward(&self, input: char) -> char {
        let mut index = to_index(input);
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
            inverse[to_index(*c)] = to_char(i);
        }

        let index = to_index(input);
        inverse[index]
    }

    fn rotate(&mut self) -> bool {
        self.position = (self.position + 1) % 26;
        self.position == 0
    }
}
