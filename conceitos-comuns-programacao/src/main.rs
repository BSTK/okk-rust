const PONTUACAO_MAXIMA: u32 = 100_000;

fn main() {

    // Dados Imutaveis
    let x = 10;
    let y = 20;
    let z = 30;

    println!("Eixo X, Y, Z = ({}, {}, {})", x, y, z);

    // Dados mutaveis
    let mut a = 100;
    let mut b = 110;
    let mut c = 120;

    println!("Eixo A, B, C = ({}, {}, {})", a, b, c);

    a = a * 2 - 1;
    b = a * 2;
    c = a + b + c;

    println!("Eixo A, B, C = ({}, {}, {})", a, b, c);

    // Uso de contantes
    println!("Pontuação maxima: {}", PONTUACAO_MAXIMA);

    // Sombreamento (Shadowing) de variaveis
    let codigo_usuario = 10_000;
    let codigo_usuario = codigo_usuario + 10;

    let espacos = "    ";
    let espacos = espacos.len(); 

    println!("Codigo Usuario: {}", codigo_usuario);
    println!("Espaços = {}", espacos);
}
