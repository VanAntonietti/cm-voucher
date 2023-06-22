mod get_data;
mod parser;

fn main() {
    let name: String = get_data::get_name();
    let date: String = get_data::get_date();
    let phone: String = get_data::get_phone();
    let value: i16 = get_data::get_value();
    if (parser::string_generetaror(name, date, phone, value) == false) {
        println!("Erro: algum dos campos está vazio.");
    }
    else {
        
    }
    return;
}

