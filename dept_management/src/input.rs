use std::io::{self, Write};
use std::slice::Iter;

pub type Menu<'a, T> = Iter<'a, MenuItem<'a, T>>;
pub type MenuRef<'a, T> = Iter<'a, MenuItemRef<'a, T>>;

#[derive(Copy, Clone, Debug)]
pub struct MenuItem<'a, T> {
    pub label: &'a str,
    pub option: T,
}

#[derive(Copy, Clone, Debug)]
pub struct MenuItemRef<'a, T> {
    pub label: &'a str,
    pub option: &'a T,
}

pub fn read_line(prompt: &str) -> String {
    loop {
        print!("{prompt} ");
        let _ = io::stdout().flush();
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim().to_string();
                if !trimmed.is_empty() {
                    break trimmed;
                }
                println!("Input cannot be empty. Please try again.");
            }
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

pub fn read_menu_option<T: Copy>(mut menu: Menu<T>, prompt: &str) -> T {
    let err_msg = "Please enter a valid option.";

    println!("");
    for (index, item) in menu.clone().enumerate() {
        println!("{}. {}", index + 1, item.label);
    }

    loop {
        let selected = read_value::<usize>(prompt, err_msg);

        // explicit check to prevent overflow
        if selected == 0 {
            println!("{err_msg}");
            continue;
        }

        match menu.nth(selected - 1) {
            Some(item) => break item.option,
            None => println!("{err_msg}"),
        }
    }
}

pub fn read_menu_option_ref<'a, T>(
    mut menu: MenuRef<'a, T>,
    prompt: &str,
) -> &'a T {
    let err_msg = "Please enter a valid option.";

    println!("");
    for (index, item) in menu.clone().enumerate() {
        println!("{}. {}", index + 1, item.label);
    }

    loop {
        let selected = read_value::<usize>(prompt, err_msg);

        // explicit check to prevent overflow
        if selected == 0 {
            println!("{err_msg}");
            continue;
        }

        match menu.nth(selected - 1) {
            Some(item) => break &item.option,
            None => println!("{err_msg}"),
        }
    }
}
