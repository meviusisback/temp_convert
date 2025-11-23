use std::io;

fn main() {
    println!("Welcome to the Celsius/Fahreneit converter\n\n");

    let conversion = conversion_input();

    if conversion == 1 {
        println!("You picked 1\n");
        let celsius = temp_input("Celsius");
        let cel_to_fahr = celsius * 1.8 + 32.0;
        println!("The temperature in Fahrenheit is {:.2}", cel_to_fahr);
    } else {
        println!("You picked 2\n");
        let fahrenheit = temp_input("Fahrenheit");
        let fahr_to_cel = (fahrenheit - 32.0) / 1.8;
        println!("The temperature in Celsius is {:.2}", fahr_to_cel);
    }
}

fn conversion_input() -> u8 {
    loop {
        println!("Choose what conversion you need to apply: \n\n");
        println!("1 - From Celsius to Fahreneit\n");
        println!("2 - From Fahreneit to Celsius\n");

        let mut conversion = String::new();

        io::stdin()
            .read_line(&mut conversion)
            .expect("Failed to read the line");

        let conversion: u8 = match conversion.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return conversion;
    }
}

fn temp_input(format: &str) -> f32 {
    loop {
        let mut temperature = String::new();
        println!("Enter the temperature in {format}");
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read the line");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return temperature;
    }
}
