use std::io;

pub fn input_option_user() -> String
{
    let mut option = String::new();

    io::stdin()
               .read_line(&mut option)
               .expect("Erro ao ler a variável option!");
    
    option
}