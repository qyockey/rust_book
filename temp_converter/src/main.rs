mod input;


#[derive(Copy, Clone, Debug)]
struct Conversion {
    source_degree_symbol: char,
    target_degree_symbol: char,
    convert: fn(f64) -> f64,
}


fn fahr_to_cel(degrees_f: f64) -> f64 {
    5.0 / 9.0 * (degrees_f - 32.0)
}


fn cel_to_fahr(degrees_c: f64) -> f64 {
    9.0 / 5.0 * degrees_c + 32.0
}


fn read_conversion_info() -> &'static Conversion {
    const NUM_CONVERSIONS: usize = 2;
    static CONVERSION_MENU: [input::MenuItem<Conversion>; NUM_CONVERSIONS] = [
        input::MenuItem {
            label: "Fahrenheight to Celsius",
            option: &Conversion {
                source_degree_symbol: 'F',
                target_degree_symbol: 'C',
                convert: fahr_to_cel,
            },
        },
        input::MenuItem {
            label: "Celsius to Fahrenheight",
            option: &Conversion {
                source_degree_symbol: 'C',
                target_degree_symbol: 'F',
                convert: cel_to_fahr,
            },
        },
    ];

    input::read_menu_option::<Conversion>(
        CONVERSION_MENU.iter(),
        "Select Conversion:"
    )
}


fn main() {
    let &Conversion {
        source_degree_symbol,
        target_degree_symbol,
        convert
    } = read_conversion_info();

    let source_degrees = input::read_value::<f64>(
        &format!("Enter temperature (°{}):", source_degree_symbol),
        "Please enter a valid decimal number."
    );
    let target_degrees = convert(source_degrees);

    println!("{source_degrees}°{source_degree_symbol} is equivalent to \
            {target_degrees}°{target_degree_symbol}.");
}


