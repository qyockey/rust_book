use std::io::{self, Write};


pub struct MenuItem<T> {
    pub message: String,
    pub data: T,
}


fn read_line(prompt: &str) -> String {
    loop {
        print!("{prompt} ");
        let _ = io::stdout().flush();
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => break input,
            Err(e) => println!("Error reading input.\n{e}"),
        }
    }
}


pub fn read_value<T: std::str::FromStr>(prompt: &str, err_msg: &str) -> T {
    loop {
        match read_line(prompt).trim().parse() {
            Ok(val) => break val,
            Err(_) => println!("{err_msg}"),
        }
    }
}


pub fn read_menu_option<T>(menu: &Vec<MenuItem<T>>) -> &T {
    let mut count: usize = 1;
    for item in menu {
        println!("{count}. {}", item.message);
        count += 1;
    }

    loop {
        let selected = read_value::<usize>("Select conversion:",
                "Please enter a valid option.");

        // explicit check to prevent overflow
        if selected == 0 {
            println!("Please enter a valid option.");
            continue;
        }

        match &menu.get(selected - 1) {
            Some(item) => break &item.data,
            None => println!("Please enter a valid option."),
        }
    }
}


