use crate::utils::{self, length_converters::*};

pub fn length_converter()
{
    println!("Agora selecione o tipo de Conversão:");
    println!("1 - [Milhas para QuilÔmetro]\n\
    2 - [Pés para Metros]\n\
    3 - [Pelegadas para Centímetros]\n\
    4 - [Jardas para Metros]\n\
    5 - [KM - HM - DAM - M - DM - CM - MM]");

    let option:u8 = utils::input_user().trim().parse().unwrap();

    match option
    {
        1 => {miles_to_kilometers::miles_to_kilometers()},
        2 => {feet_to_meters::feet_to_meters()},
        /*3 => {inches_to_centimeters::inches_to_centimeters()},
        4 => {yards_to_meters::yards_to_meters()},
        5 => {kilometers_to_millimeters::kilometers_to_milimeters()},*/
        _=> {println!("Essa opção não é valida!")},
    }


}