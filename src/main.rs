fn main() {
    println!("Enter the degree:");
    let mut temp = String::new();
    std::io::stdin().read_line(&mut temp);
    let float_temp: f64 = temp.trim().parse().expect("ERROR");
    if float_temp > -273.0 && float_temp <= 6000.0 {
        if float_temp >= 100.0 {
            println!("Steam");
        } else if float_temp < 0.0 {
            println!("Ice");
        } else {
            println!("Water");
        }
    } else {
        println!("The Temperature is incorrect.");
    }
}
