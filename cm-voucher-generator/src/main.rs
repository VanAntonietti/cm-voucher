mod get_data;
mod parser_string;
mod image_generator;

fn main() {
    let name: String = get_data::get_name();
    let date: String = get_data::get_date();
    let phone: String = get_data::get_phone();
    let value: i16 = get_data::get_value();
    if parser_string::string_generetaror(name, date, phone, value) == false {
        println!("Erro: algum dos campos está vazio.");
    }
    else {
        image_generator::image_generator(data);
    }
    return;
}

