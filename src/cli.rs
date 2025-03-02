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
