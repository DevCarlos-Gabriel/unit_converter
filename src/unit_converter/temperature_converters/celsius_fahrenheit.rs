use std::io::{self,Write};
use crate::constants;
use crate::input_user::input_user;

pub fn celsius_fahrenheit(){
    println!("Pronto! Agora que você está aqui...\n\
    Escolha qual a ordem de conversão:");

    println!("1 - [Fahrenheit para Celsius]\n\
    2 - [Celsius para Fahrenheit]\n\
    ");
    
    let option:u8 = input_user().trim().parse().unwrap();

    match option{
        1 => {fahrenheit_to_celsius()},
        2 => {celsius_to_fahrenheit()},
        _=> {println!("Essa opção não é valida!");}
    }

    fn fahrenheit_to_celsius(){
        print!("Informe a temperatura em Fahrenheit: ");
        io::stdout().flush().unwrap();

        let fahrenheit:f64 = input_user().trim().parse().unwrap();

        let convert_to_celsius = (fahrenheit - constants::SET_POINT_C_TO_K) * constants::INVERSION_FACTOR;

        println!("Convertendo fica assim:\n\
        {} Fahrenheit equivale a {} Celsius.
        ", fahrenheit, convert_to_celsius);
    }

    fn celsius_to_fahrenheit(){
        print!("Informe a temperatura em Celsius: ");
        io::stdout().flush().unwrap();

        let celsius:f64 = input_user().trim().parse().unwrap();

        let convert_to_fahrenheit = constants::SCALE_FACTOR * celsius + constants::SET_POINT_C_TO_K;

        println!("Convertendo fica assim:
        {} Celsius equivalem a {} Fahrenheit
        ", celsius, convert_to_fahrenheit)
    }
}