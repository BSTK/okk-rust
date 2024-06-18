fn main() {
    println!("--- Controle de Fluxo ---");

    let numero = 4000;

    if numero % 4 == 0 {
        println!("Número resto em 4!");
    }
    else if numero % 3 == 0 {
        println!("Número resto em 3!");
    }
    else if numero % 2 == 0 {
        println!("Número resto em 2!");
    }
    else if numero % 1 == 0 {
        println!("Número resto em 1!");
    }
    else {
        println!("Deu em nada!");
    }
}
