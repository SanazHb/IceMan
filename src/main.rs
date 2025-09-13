fn main() {
    println!("Enter the degree:");
    let mut Temp = String::new();
    std::io::stdin().read_line(&mut Temp);
    let float_Temp: f64 = Temp.trim().parse().expect("ERROR");
    if float_Temp > -273.0 && float_Temp <= 6000.0 {
        if float_Temp >= 100.0 {
            println!("Steam");
        } else if float_Temp < 0.0 {
            println!("Ice");
        } else {
            println!("Water");
        }
    } else {
        println!("The Temperature is incorrect.");
    }
}
