// Integrando conversores e o input para o usuário
use crate::utils;

pub fn temperature_converter(){
    println!("Agora selecione o tipo de conversão:");
    println!("1 - [Celsius e Fahrenheit]\n\
    2 - [Kelvin e Celsius]\n\
    3 - [Kelvin e Fahrenheit]\n\
    ");

    let option:u8 = utils::input_user().trim().parse().unwrap();

    //Escolhendo unidades para converter
    match option{
        1 => {utils::celsius_fahrenheit::celsius_fahrenheit()},
        2 => {utils::kelvin_celsius::kelvin_celsius()},
        3 => {utils::kelvin_fahrenheit::kelvin_fahrenheit()},
        _=> {println!("Essa opção não é valida!");}
    }
}