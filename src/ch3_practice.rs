use std::io;

pub fn temp_convert(){
    println!("Input a temp to convert to Celsius");

    let mut degf = String::new();

    io::stdin()
        .read_line(&mut degf)
        .expect("Failed to read line");

    let degf: i32 = match degf.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };

    let degc:i32 = (degf-32)*5/9;

    println!("{degf} -> {degc}");
}