// Integrando conversores e o input para o usuário
use crate::{terminal_utils, utils::{self, temperature_converters::*}};

pub fn temperature_converter(){
    println!("Agora selecione o tipo de conversão:");
    println!("1 - [Celsius e Fahrenheit]\n\
    2 - [Kelvin e Celsius]\n\
    3 - [Kelvin e Fahrenheit]\n\
    4 - [Voltar]
    ");

    let option:u8 = utils::input_user().trim().parse().unwrap();

    //Escolhendo unidades para converter
    match option{
        1 => {celsius_fahrenheit::celsius_fahrenheit()},
        2 => {kelvin_celsius::kelvin_celsius()},
        3 => {kelvin_fahrenheit::kelvin_fahrenheit()},
        4 =>
        {
            println!("Voltando...");

            terminal_utils::sleep_terminal(2);
            
            terminal_utils::clear_terminal();
        },

        _=> {println!("Essa opção não é valida!");}
    }
}