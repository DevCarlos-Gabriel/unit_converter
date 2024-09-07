mod input_user; // Importando o módulo input_user

pub mod constants; // Importando o módulo constants

mod unit_converter; // Importando o módulo unit_converters

mod utils; // Importando o módulo utils

fn main() {
    println!("Olá, informe uma das seguintes opções de conversor de unidades:");

    println!("1 - [Temperatura]\n\
    2 - [Comprimento]\n\
    3 - [Massa]\n\
    4 - [Volume]\n\
    5 - [Tempo]\n\
    6 - [Velocidade]\n\
    7 - [Área]\n\
    8 - [Energia]\n\
    9 - [Pressão]\n\
    ");

    let option:u8 = utils::input_user().trim().parse().unwrap();

    // Verificando qual a conversão que o usuário quer.
    match option
    {
        1 => {utils::input_temperature_converter::temperature_converter()},
        /*2 => {length_converter()},
        3 => {mass_converter()},
        4 => {volume_converter()},
        5 => {time_converter()},
        6 => {speed_converter()},
        7 => {area_converter()},
        8 => {energy_converter()},
        9 => {pressure_converter()},*/
        _=> {println!("Esta opção não é valida!");}
    }
}