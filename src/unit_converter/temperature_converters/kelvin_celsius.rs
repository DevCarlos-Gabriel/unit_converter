use crate::utils::{self, constants::*,io, Write};

pub fn kelvin_celsius(){
    println!("Pronto! Agora que você está aqui...\n\
    Escolha qual a ordem de conversão:");

    println!("1 - [Kelvin para Celsius]\n\
    2 - [Celsius para Kelvin]\n\
    ");

    let option:u8 = utils::input_user().trim().parse().unwrap();

    match option{
        1 => {kelvin_to_celsius()},
        2 => {celsius_to_kelvin()},
        _=> {println!("Essa opção não é valida!")},
    }

    // Calculando conversão de kelvin para celsius
    fn kelvin_to_celsius(){
        print!("Informe a temperatura em Kelvin: ");
        io::stdout().flush().unwrap();

        let kelvin: f64 = utils::input_user().trim().parse().unwrap();

        let convert_to_celsius:f64 = kelvin - temperature_constants::KELVIN_CONSTANT;

        println!("Convertendo fica assim:\n\
        {} Kelvin equivale a {} Celsius.\n\
        ", kelvin, convert_to_celsius);
    }

    // Calculando conversão de celsius para kelvin
    fn celsius_to_kelvin(){
        print!("Informe a temperatura em Celsius: ");
        io::stdout().flush().unwrap();
        
        let celsius:f64 = utils::input_user().trim().parse().unwrap();

        let convert_to_kelvin:f64 = celsius + temperature_constants::KELVIN_CONSTANT;

        println!("Convertendo fica assim:\n\
        {} Celsius equivale a {} Kelvin.\n\
        ", celsius, convert_to_kelvin);
    }
}