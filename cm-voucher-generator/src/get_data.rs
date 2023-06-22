use std::io;

pub fn get_name () -> String {
    let mut line = String::new();
    println!("Entre com o nome da cliente:");
    let _name = io::stdin().read_line(&mut line).unwrap();
    return line;
}

pub fn get_date () -> String {
    let mut line = String::new();
    println!("Entre com a data de atendimento no formato xx/xx/xx:");
    let _date = io::stdin().read_line(&mut line).unwrap();
    return line;
}

pub fn get_phone() -> String {
    let mut line = String::new();
    println!("Entre com o telefone da cliente no formato xx xxxxx-xxxx:");
    let _phone = io::stdin().read_line(&mut line).unwrap();
    return line;
}

pub fn get_value() -> i16 {
    let mut line = String::new();
    println!("Entre com o valor do voucher:");
    let _value = io::stdin().read_line(&mut line).unwrap();
    let value: i16 = line.trim().parse().unwrap();
    return value;
}