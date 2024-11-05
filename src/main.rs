mod input_user; // Importando o módulo input_user

pub mod constants; // Importando o módulo constants

mod unit_converter; // Importando o módulo unit_converters

mod utils; // Importando o módulo utils

pub mod terminal_utils;

fn main() {
    loop
    {
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
        10 - [Sair do Programa]\n\
        ");
    
        let option:u8 = utils::input_user().trim().parse().unwrap();
    
        // Verificando qual a conversão que o usuário quer.
        match option
        {
            1 => {utils::temperature_converters::input_temperature_converter::temperature_converter()},
            2 => {utils::length_converters::input_length_converter::length_converter();},
            /*3 => {mass_converter()},
            4 => {volume_converter()},
            5 => {time_converter()},
            6 => {speed_converter()},
            7 => {area_converter()},
            8 => {energy_converter()},
            9 => {pressure_converter()},*/
            10 =>
            {
                println!("[Programa Finalizando] Obrigado pela visita! Volte sempre!");

                terminal_utils::sleep_terminal(5);

                terminal_utils::clear_terminal();

                break;
            },
            _=> {println!("Esta opção não é valida!");}
        }
    }
}