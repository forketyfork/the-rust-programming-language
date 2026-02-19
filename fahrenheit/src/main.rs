use std::io;

fn main() {
    loop {
        println!("Enter:\n- 0 to exit,\n- 1 to convert F to C,\n- 2 to convert C to F");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read the choice");
        match choice.trim() {
            "0" => break,
            "1" => fahr_to_cels(),
            "2" => cels_to_fahr(),
            _ => println!("Wrong input.")
        }
    }
}

fn fahr_to_cels() {
    println!("Enter temperature in Fahrenheit:");
    let mut fahr = String::new();
    io::stdin()
        .read_line(&mut fahr)
        .expect("Failed to read the value.");

    let fahr: f64 = fahr.trim().parse().expect("Failed to parse the number.");
    let cels = (fahr - 32.0) * 5.0 / 9.0;
    println!("The result in Celsius is: {cels}");
}

fn cels_to_fahr() {
    println!("Enter temperature in Celsius:");
    let mut cels = String::new();
    io::stdin()
            .read_line(&mut cels)
            .expect("Failed to read the value.");

    let cels: f64 = cels.trim().parse().expect("Failed to parse the number.");
    let fahr = 32.0 + cels * 9.0 / 5.0;
    println!("The result in Fahrenheit is: {fahr}");
}