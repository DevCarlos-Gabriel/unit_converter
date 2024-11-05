use std::{env, process::Command, thread, time::Duration};

pub fn clear_terminal()
{
    // Verifica qual é o sistema operacional
    let os = env::consts::OS;

    match os {
        "linux" | "macos" => {
            // Para Linux e macOS
            Command::new("clear")
                .status()
                .expect("Falha ao executar o comando");
        }
        "windows" => {
            // Para Windows
            Command::new("cls")
                .status()
                .expect("Falha ao executar o comando");
        }
        _ => {
            println!("Sistema operacional não suportado para limpeza de terminal.");
        }
    }
}

pub fn sleep_terminal(secs:u64)
{
    thread::sleep(Duration::new(secs,0));
}