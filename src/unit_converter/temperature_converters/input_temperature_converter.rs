use crate::input_user::input_user;

// Integrando os modulos dos conversores

pub use crate::unit_converter::temperature_converters::{celsius_fahrenheit::celsius_fahrenheit, kelvin_celsius::kelvin_celsius, kelvin_fahrenheit::kelvin_fahrenheit};

pub fn temperature_converter(){
    println!("Agora selecione o tipo de conversão:");
    println!("1 - [Kelvin e Celsius]\n\
    2 - [Celsius e Fahrenheit]\n\
    3 - [Kelvin e Fahrenheit]\n\
    ");

    let option:u8 = input_user().trim().parse().unwrap();

    //Escolhendo unidades para converter
    match option{
        1 => {kelvin_celsius()},
        2 => {celsius_fahrenheit()},
        3 => {kelvin_fahrenheit()},
        _=> {println!("Essa opção não é valida!");}
    }
}