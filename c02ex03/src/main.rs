use std::io;
use std::io::prelude::*;

fn main(){
    let mut valor = String::new();
    let vlr: i64;

    print!("Informe a sua idade: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor).unwrap();

    vlr = valor.trim().parse::<i64>().unwrap();

    print!("");
    print!("Sua idade é: {} ", vlr);

    println!("");
    print!("Tecle <Enter> para encerra o programa...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
