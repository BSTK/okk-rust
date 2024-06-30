fn main() {
    controle_fluxo_if();
    retorno_expreessao_if();
    lacos_repeticao_loop();
    lacos_repeticao_while();
    lacos_repeticao_for();
}

fn lacos_repeticao_for() {
    print!("\n\n");
    println!("------------------------");
    println!("- USANDO O LAÇO for    -");
    println!("------------------------");

    let itens = [10, 20, 30, 40, 50, 60, 70, 80, 90, 00];
    for item in itens.iter() {
        println!("Laço: for! Item = {item}");
    }

    print!("\n");

    // Iterando com itens reverso
    for item in (1..11).rev() {
        println!("Laço for! Item = {item}");
    }
}

fn lacos_repeticao_while() {
    print!("\n\n");
    println!("------------------------");
    println!("- USANDO O LAÇO while  -");
    println!("------------------------");

    let mut contador: i16 = 0;

    while contador < 10 {
        println!("Laço: While");
        contador = contador + 1;
    }
}

fn lacos_repeticao_loop() {
    print!("\n\n");
    println!("------------------------");
    println!("- USANDO O LAÇO loop   -");
    println!("------------------------");

    let mut contador: i16 = 0;
    loop {
        println!("Laço: loop!");
        contador = contador + 1;

        if contador == 10 {
            break;
        }
    }
}

fn retorno_expreessao_if() {
    let condicao_booleana = false;
    let retorno_expressao_booleana = if condicao_booleana {
        println!("------------------------\nCondição VERDADEIRA!!\n------------------------");
        5
    } else {
        println!("------------------------\nCondição FALSA!!\n------------------------");
        -1
    };

    print!("Retorno da condição = {}", retorno_expressao_booleana);
}

fn controle_fluxo_if() {
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
