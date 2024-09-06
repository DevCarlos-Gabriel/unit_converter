use std::io;

pub fn input_user() -> String
{
    let mut input_value = String::new();

    io::stdin()
               .read_line(&mut input_value)
               .expect("Erro ao ler a variável input_value!");
    
    input_value
}