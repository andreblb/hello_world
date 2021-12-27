use std::io;
use std::io::prelude::*;

fn main(){
    let mut idade = String::new();
    let ano: i64;

    print!("Informe a sua idade: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut idade).unwrap();

    ano = idade.trim().parse::<i64>().unwrap();

    print!("");
    print!("Sua idade é: {} ", ano);

    println!("");
    print!("Tecle <Enter> para encerra o programa...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
