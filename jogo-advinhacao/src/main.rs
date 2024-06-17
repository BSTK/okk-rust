extern crate rand;

use rand::Rng;
use std::{cmp::Ordering, io::stdin};

fn main() {
    println!("-- Adivinhe o número --");

    loop {
        println!("Digite seu palpite: ");
        let mut palpite = String::new();

        stdin()
            .read_line(&mut palpite)
            .expect("Erro ao ler palpite digitado!");

        let palpite: u32 = match palpite.trim().parse() {
                Ok(palpite) => palpite,
                Err(_) => {
                    println!("Digite apenas número!");
                    continue;
                }
            };
        
        let numero_secreto = rand::thread_rng()
            .gen_range(1..11);

        println!("Número escolhido: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!\nO número esperado é: {}", numero_secreto),
            Ordering::Greater => println!("Muito Alto!\nO número esperado é: {}", numero_secreto),
            Ordering::Equal => {
                println!("AccertooOU!");
                break;
            }
        }
    }
}
