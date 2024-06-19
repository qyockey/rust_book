use std::io;
use std::cmp::Ordering;


fn read_positive_int() -> u32 {
    loop {
        let mut input = String::new();
        println!("Which fibonacci number would you like to calculate?");

        match io::stdin().read_line(&mut input) {
            Ok(num_bytes) => println!("{num_bytes} bytes read."),
            Err(_) => {
                println!("Error reading input.");
                continue;
            },
        }

        match input.trim().parse() {
            Ok(val) => return val,
            Err(_) => {
                println!("Please enter a positive integer.");
                continue;
            },
        }

    }
}


enum FibError {
    UsizeConversion,
    OutOfBounds,
}


fn get_fib(n: u32) -> Result<u32, FibError> {
    let n: usize = match n.try_into() {
        Ok(val) => val,
        Err(_) => return Err(FibError::UsizeConversion),
    };

    let mut fibs: Vec<u32> = Vec::new();
    fibs.push(0);
    fibs.push(1);

    for i in 2..=n {
        println!("{i}");
        let num = match (fibs.get(i - 2), fibs.get(i - 1)) {
            (Some(x), Some(y)) => x + y,
            _ => return Err(FibError::OutOfBounds),
        };
        fibs.push(num);
    }

    Ok(fibs[n])
}


fn main() {
    const MAX_FIB: u32 = 47;
    let n = loop {
        let input = read_positive_int();
        match input.cmp(&MAX_FIB) {
            Ordering::Less | Ordering::Equal => break input,
            Ordering::Greater => {
                println!("Please enter a value less than or equal to \
                         {MAX_FIB}");
            }
        }
    };

    let fib = match get_fib(n) {
        Ok(val) => val,
        Err(_) => {
            println!("Error calculating fibonacci value.");
            return;
        },
    };
    println!("{fib}");
}


