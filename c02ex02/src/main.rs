use std::io;
use std::io::prelude::*;

fn main() {
    let mut nome = String::new();

    print!("Informe seu nome: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nome).expect("Entrada incorreta...");

    if let Some('\n') = nome.chars().next_back(){
        nome.pop();
    }

    if  let Some('\r') = nome.chars().next_back(){

        nome.pop();
    }

    println!("");
    println!("Olá, {}\n ", nome);
    print!("Tecle <Enter> para sair... ");

    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
