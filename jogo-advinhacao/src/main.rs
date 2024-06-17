extern crate rand;

use std::{cmp::Ordering, io::stdin};
use rand::Rng;

fn main() {
    println!("-- Adivinhe o número --");

    println!("Digite seu palpite: ");
    let mut palpite = String::new();    
    let numero_secreto = rand::thread_rng()
       .gen_range(1..101)
       .to_string();
    
    stdin()
    .read_line(&mut palpite)
    .expect("Erro ao ler palpite digitado!");

    println!("Número escolhido: {}", palpite);

    match palpite.cmp(&numero_secreto) {
        Ordering::Less => println!("Muito baixo!"),
        Ordering::Greater => println!("Muito Alto!"),
        Ordering::Equal => println!("AccertooOU!"),  
    }
}

// https://rust-br.github.io/rust-book-pt-br/ch02-00-guessing-game-tutorial.html
