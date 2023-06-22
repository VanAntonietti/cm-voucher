use sha2::{Sha256, Digest};

mod data {
    pub struct Data {
        name: String,
        date: String,
        phone: String,
        hash: u8,
        value: i16,
    }

    impl Data {
        pub fn new(name: String, date: String, phone: String, hash: u8, value: i16) -> Data {
            Data {
                name,
                date,
                phone,
                hash,
                value,
            }
        }
    }
}

pub fn string_generetaror(name: String, date: String, phone: String, value: i16) -> bool {
    if (name.len() == 0) || (date.len() == 0) || (phone.len() == 0) {
        println!("Erro: algum dos campos está vazio.");
        return false;
    }
    else if(&name[name.len()-1..] == " ") || (&date[date.len()-1..] == " ") || (&phone[phone.len()-1..] == " ") {
        println!("Erro: dado inválido, digite sem epaços no final.");
        return false;
    }

    let string: String = format!("{}{}", name, phone);
    println!("String gerada: {}", string);
    let mut hasher = Sha256::new();
    hasher.update(string.as_bytes());
    let mut hash: u8 = hasher.finalize();
    println!("Hash gerado: {:x}", hash);
    let data = data::Data::new(name, date, phone, hash, value);

return true;
}