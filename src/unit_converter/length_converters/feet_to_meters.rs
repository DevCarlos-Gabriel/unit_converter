use crate::utils::{self, constants::length_constants::*, io, Write};

pub fn feet_to_meters()
{
    println!("Pronto! Agora que você está aqui...\n\
    Escolha a ordem de conversão:");

    println!("1 - [Pés para Metros]\n\
    2 - [Metros para Pés]
    ");

    let option:u8 = utils::input_user().trim().parse().unwrap();

    match option
    {
        1 => {fe_to_met()},
        2 => {met_to_fe()},
        _ => {println!("Opção invalida!")}
    }

    fn fe_to_met()
    {
        print!("Informe o comprimento em Pés: ");
        io::stdout().flush().unwrap();

        let feet:f64 = utils::input_user().trim().parse().unwrap();

        let convert_to_meters = feet * F_TO_M;

        println!("Convertendo fica assim:
        \n\
        {} Pés equivale a {} Metros.
        ", feet, convert_to_meters);
    }

    fn met_to_fe()
    {
        print!("Informe o comprimento em Metros: ");
        io::stdout().flush().unwrap();

        let meters:f64 = utils::input_user().trim().parse().unwrap();

        let convert_to_meters = meters/F_TO_M;

        println!("Convertendo fica assim:
        \n\
        {} Metros equivale a {} Pés.
        ", meters, convert_to_meters);
    }
}