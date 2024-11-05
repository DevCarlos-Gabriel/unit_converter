use crate::utils::{self, constants::length_constants::*, io, Write};

pub fn miles_to_kilometers()
{
    println!("Pronto! Agora que você está aqui...\n\
    Escolha a ordem de conversão:");

    println!("1 - [Milhas para Quilômetros]\n\
    2 - [Quilômetros para Milhas]\n\
    ");

    let option:u8 = utils::input_user().trim().parse().unwrap();

    match option
    {
        1 => {mil_to_kil()},
        2 => {kil_to_mil()},
        _ => {println!("Opção invalida!")}
    }

    fn mil_to_kil()
    {
        print!("Informe o comprimento em Milhas: ");
        io::stdout().flush().unwrap();

        let miles:f64 = utils::input_user().trim().parse().unwrap();

        let convert_to_kilometers = miles * M_TO_KM;

        println!("Convertendo fica assim:
        \n\
        {} Milhas equivale a {} Quilômetros.
        ", miles, convert_to_kilometers);

    }

    fn kil_to_mil()
    {
        print!("Informe o comprimento em Quilômetros: ");
        io::stdout().flush().unwrap();

        let kilometers:f64 = utils::input_user().trim().parse().unwrap();

        let convert_to_miles = kilometers/M_TO_KM;

        println!("Convertendo fica assim:
        \n\
        {} Quilômetros equivale a {} Milhas.
        ", kilometers, convert_to_miles);
    }
}