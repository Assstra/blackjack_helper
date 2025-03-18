use std::io::stdin;

pub fn input_cli() -> u8 {
    let input = loop {
        let mut s: String = String::new();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");

        let input = s.trim().parse::<u8>();
        match input {
            Ok(input) => {
                println!("You entered {}", input);
                break input;
            }
            Err(..) => {
                println!("Please enter a number!");
            }
        };
    };
    input
}

pub fn hit_or_stand_input() -> String {
    let input = loop {
        let mut s: String = String::new();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");

        let input = s.trim().parse::<String>();
        match input {
            Ok(input) => {
                println!("You entered {}", input);
                break input;
            }
            Err(..) => {
                println!("Please enter a string!");
            }
        };
    };
    input
}

pub fn split_input() -> String {
    let input = loop {
        let mut s: String = String::new();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");

        let input = s.trim().parse::<String>();
        match input {
            Ok(input) => {
                println!("You entered {}", input);
                break input;
            }
            Err(..) => {
                println!("Please enter a string!");
            }
        };
    };
    input
}
