// declarando contantes para as conversões
mod constants;

// declarando módulo que tem input
mod input_user;
// reexportando o módulo a cima
pub use input_user::input_user;

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

    let option:u8 = input_user().trim().parse().unwrap();

    // Função que verifica qual o conversor que o usuário quer

    unit_converter(option);

}

// declarando módulo onde tem os conversores de unidades
mod unit_converter;
// reexportando esse módulo
pub use unit_converter::temperature_converters::input_temperature_converter::temperature_converter;

fn unit_converter(opt: u8)
{
    match opt
    {
        1 => {temperature_converter()},
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