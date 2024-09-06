use std::io::{self,Write};
use crate::constants;
use crate::input_user::input_user;

pub fn kelvin_fahrenheit()
{
    println!("Pronto! Agora que você está aqui...\n\
    Escolha a ordem de conversão:");

    println!("1 - [Fahrenheit para Kelvin]\n\
    2 - [Kelvin para Fahrenheit]\n\
    ");

    let option:u8 = input_user().trim().parse().unwrap();

    match option{
        1 => {fahrenheit_to_kelvin()},
        2 => {kelvin_to_fahrenheit()},
        _=> {println!("Essa opção não é valida!");},
    }

    fn fahrenheit_to_kelvin(){
        print!("Informe a temperatura em Fahrenheit: ");
        io::stdout().flush().unwrap();

        let fahrenheit:f64 = input_user().trim().parse().unwrap();

        let convert_to_kelvin = (fahrenheit + constants::SET_POINT_F_TO_K) * constants::INVERSION_FACTOR;

        println!("Convertendo fica assim:\n\
        {} Fahrenheit equivale a {} Kelvin.
        ", fahrenheit, convert_to_kelvin);
    }

    fn kelvin_to_fahrenheit(){
        print!("Informe a temperatura em Kelvin: ");
        io::stdout().flush().unwrap();

        let kelvin:f64 = input_user().trim().parse().unwrap();

        let convert_to_fahrenheit = (kelvin * constants::SCALE_FACTOR) - constants::SET_POINT_F_TO_K;

        println!("Convertendo fica assim:\n\
        {} Kelvin equivale a {} Fahrenheit.
        ", kelvin, convert_to_fahrenheit);
    }
}