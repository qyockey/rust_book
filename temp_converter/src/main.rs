mod input;


enum Conversion {
    FahrenheightToCelsius,
    CelsiusToFahrenheight,
}


fn fahr_to_cel(degrees_f: f64) -> f64 {
    5.0 / 9.0 * (degrees_f - 32.0)
}


fn cel_to_fahr(degrees_c: f64) -> f64 {
    9.0 / 5.0 * degrees_c + 32.0
}


fn main() {
    let conversion_menu = vec![
        input::MenuItem { message: String::from("Fahrenheight to Celsius"),
                   data: Conversion::FahrenheightToCelsius},
        input::MenuItem { message: String::from("Celsius to Fahrenheight"),
                   data: Conversion::CelsiusToFahrenheight},
    ];

    let conversion = input::read_menu_option::<Conversion>(&conversion_menu);

    let src_degree_symbol: char;
    let target_degree_symbol: char;
    let convert: fn(f64) -> f64;
    match conversion {
        Conversion::FahrenheightToCelsius => {
            src_degree_symbol = 'F';
            target_degree_symbol = 'C';
            convert = fahr_to_cel;
        },
        Conversion::CelsiusToFahrenheight => {
            src_degree_symbol = 'C';
            target_degree_symbol = 'F';
            convert = cel_to_fahr;
        },
    };

    let src_degrees = input::read_value::<f64>(&format!(
            "Enter temperature (°{src_degree_symbol}):"),
            &String::from("Please enter a valid decimal number."));
    let target_degrees = convert(src_degrees);
    println!("{src_degrees}°{src_degree_symbol} is equivalent to \
             {target_degrees}°{target_degree_symbol}.");
}


