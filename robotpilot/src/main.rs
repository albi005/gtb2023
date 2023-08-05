fn main() {
    let input = ";TURN45;GO 2; T U R N -30.22 ;;; GO -1.1;";

    let commands = input
        .split(';')
        .map(|s| s.chars()
                  .filter(|c| !c.is_whitespace())
                  .collect::<String>());

    let mut ship = Ship::new();
    
    for command in commands {
        if command.is_empty() {
            continue;
        }

        let argument_string;
        if command.starts_with("TURN") {
            argument_string = &command[4..];
        } else if command.starts_with("GO") {
            argument_string = &command[2..];
        } else {
            panic!("Unknown command {}", command);
        }

        if let Ok(argument) = argument_string.parse::<f64>() {
            if command.starts_with("TURN") {
                ship.turn_degrees(argument);
            } else {
                ship.go(argument);
            }
        } else {
            panic!("Could not parse argument {} for command {}", argument_string, command);
        }
    }

    ship.print();
}

struct Ship {
    x: f64,
    y: f64,
    angle: f64,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            x: 0.0,
            y: 0.0,
            angle: 0.0,
        }
    }
    fn turn_degrees(&mut self, angle: f64) {
        self.angle += angle.to_radians();
    }
    fn go(&mut self, distance: f64) {
        self.x += self.angle.cos() * distance;
        self.y += self.angle.sin() * distance;
    }
    fn print(&self) {
        println!("({:.2}, {:.2})", self.x, self.y);
    }
}

