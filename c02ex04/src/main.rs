use std::io;
use std::io::prelude::*;

fn main() {
    let mut hora_trab = String::new();
    let mut vlr_hora = String::new();
    let mut percent_desc = String::new();

    let ht: f64;
    let vh: f64;
    let pd: f64;
    let td: f64;
    let sb: f64;
    let sl: f64;

    print!("Entre com a quantidade de trabalhada?  ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut hora_trab).unwrap();
    ht = hora_trab.trim().parse::<f64>().unwrap();


    print!("Qual o valor da hora trabalhada  ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut vlr_hora).unwrap();
    vh = vlr_hora.trim().parse::<f64>().unwrap();

    print!("Qual o percentual do desconto?  ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut percent_desc).unwrap();
    pd = percent_desc.trim().parse::<f64>().unwrap();


    sb = ht * vh;
    td = (pd / 100.) * sb;
    sl = sb - td;

    print!("");
    println!("Salario Bruto....:{}", sb);
    println!("Desconto.........:{}", td);
    println!("Salario liquido..:{}", sl);

    println!("");
    print!("Tecle <Enter> para sair...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();


}