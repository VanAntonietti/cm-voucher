use sha2::{Sha256, Digest};

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
    let hash = hasher.finalize();
    println!("Hash gerado: {:x}", hash);

    struct Data {
        name: String,
        date: String,
        phone: String,
        hash: String,
        value: i16,
    }

return true;
}