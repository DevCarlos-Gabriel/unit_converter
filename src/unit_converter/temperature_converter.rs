use std::io::{self,Write};
use crate::input_user::input_option_user;
use crate::constants;

pub fn temperature_converter(){
    println!("Agora selecione o tipo de conversão:");
    println!("1 - Kelvin e Celsius\n\
    2 - Celsius e Fahrenheit\n\
    3 - Kelvin e Fahrenheit\n\
    ");

    let option:u8 = input_option_user().trim().parse().unwrap();

    //Escolhendo unidades para converter
    match option{
        1 => {kelvin_celsius()},
        2 => {celsius_fahrenheit()},
        3 => {kelvin_fahrenheit()},
        _=> {println!("Essa opção não é valida!");}
    }

    fn kelvin_celsius(){
        println!("Pronto! Agora que você está aqui...\n\
        Escolha qual a ordem de conversão:");

        println!("1 - Kelvin para Celsius\n\
        2 - Celsius para Kelvin\n\
        ");

        let option:u8 = input_option_user().trim().parse().unwrap();

        match option{
            1 => {kelvin_to_celsius()},
            2 => {celsius_to_kelvin()},
            _=> {println!("Essa opção não é valida!")},
        }

        // Calculando conversão de kelvin para celsius
        fn kelvin_to_celsius(){
            print!("Informe a temperatura em Kelvin: ");
            io::stdout().flush().unwrap();

            let mut kelvin = String::new();

            io::stdin()
                       .read_line(&mut kelvin)
                       .expect("Erro ao ler a variável kelvin!");
            
            let kelvin: f64 = kelvin.trim().parse().unwrap();

            let convert_to_celsius:f64 = kelvin - constants::KELVIN_CONSTANT;

            println!("Convertendo fica assim:\n\
            {} Kelvin equivale a {} Celsius.\n\
            ", kelvin, convert_to_celsius);
        }

        // Calculando conversão de celsius para kelvin
        fn celsius_to_kelvin(){
            print!("Informe a temperatura em Celsius: ");
            io::stdout().flush().unwrap();

            let mut celsius = String::new();

            io::stdin()
                       .read_line(&mut celsius)
                       .expect("Erro ao ler a variável celsius!");
            
            let celsius:f64 = celsius.trim().parse().unwrap();

            let convert_to_kelvin:f64 = celsius + constants::KELVIN_CONSTANT;

            println!("Convertendo fica assim:\n\
            {} Celsius equivale a {} Kelvin.\n\
            ", celsius, convert_to_kelvin);
        }
    }
    
    fn celsius_fahrenheit(){
        println!("Pronto! Agora que você está aqui...\n\
        Escolha qual a ordem de conversão:");

        println!("1 - Fahrenheit para Celsius\n\
        2 - Celsius para Fahrenheit\n\
        ");
        
        let option:u8 = input_option_user().trim().parse().unwrap();

        match option{
            1 => {fahrenheit_to_celsius()},
            2 => {celsius_to_fahrenheit()},
            _=> {println!("Essa opção não é valida!");}
        }

        fn fahrenheit_to_celsius(){
            print!("Informe a temperatura em Fahrenheit: ");
            io::stdout().flush().unwrap();

            let mut fahrenheit = String::new();

            io::stdin()
                       .read_line(&mut fahrenheit)
                       .expect("Erro ao ler a variável fahrenheit!");

            let fahrenheit:f64 = fahrenheit.trim().parse().unwrap();

            let convert_to_celsius = (fahrenheit - constants::SET_POINT_C_TO_K) * constants::INVERSION_FACTOR;

            println!("Convertendo fica assim:\n\
            {} Fahrenheit equivale a {} Celsius.
            ", fahrenheit, convert_to_celsius);
        }

        fn celsius_to_fahrenheit(){
            print!("Informe a temperatura em Celsius: ");
            io::stdout().flush().unwrap();

            let mut celsius = String::new();

            io::stdin()
                       .read_line(&mut celsius)
                       .expect("Erro ao ler a variável Celsius.");

            let celsius:f64 = celsius.trim().parse().unwrap();

            let convert_to_fahrenheit = constants::SCALE_FACTOR * celsius + constants::SET_POINT_C_TO_K;

            println!("Convertendo fica assim:
            {} Celsius equivalem a {} Fahrenheit
            ", celsius, convert_to_fahrenheit)
        }
    }

    fn kelvin_fahrenheit()
    {
        println!("Pronto! Agora que você está aqui...\n\
        Escolha a ordem de conversão:");

        println!("1 - Fahrenheit para Kelvin\n\
        2 - Kelvin para Fahrenheit\n\
        ");

        let option:u8 = input_option_user().trim().parse().unwrap();

        match option{
            1 => {fahrenheit_to_kelvin()},
            2 => {kelvin_to_fahrenheit()},
            _=> {println!("Essa opção não é valida!");},
        }

        fn fahrenheit_to_kelvin(){
            print!("Informe a temperatura em Fahrenheit: ");
            io::stdout().flush().unwrap();

            let mut fahrenheit = String::new();

            io::stdin()
                       .read_line(&mut fahrenheit)
                       .expect("Erro ao ler a variável fahrenheit!");

            let fahrenheit:f64 = fahrenheit.trim().parse().unwrap();

            let convert_to_kelvin = (fahrenheit + constants::SET_POINT_F_TO_K) * constants::INVERSION_FACTOR;

            println!("Convertendo fica assim:\n\
            {} Fahrenheit equivale a {} Kelvin.
            ", fahrenheit, convert_to_kelvin);
        }

        fn kelvin_to_fahrenheit(){
            print!("Informe a temperatura em Kelvin: ");
            io::stdout().flush().unwrap();

            let mut kelvin = String::new();

            io::stdin()
                       .read_line(&mut kelvin)
                       .expect("Erro ao ler a variável fahrenheit!");

            let kelvin:f64 = kelvin.trim().parse().unwrap();

            let convert_to_fahrenheit = (kelvin * constants::SCALE_FACTOR) - constants::SET_POINT_F_TO_K;

            println!("Convertendo fica assim:\n\
            {} Kelvin equivale a {} Fahrenheit.
            ", kelvin, convert_to_fahrenheit);
        }
    }
}