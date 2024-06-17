const PONTUACAO_MAXIMA: u32 = 100_000;

fn main() {

    dados_imutaveis();
    dados_mutaveis();
    constantes();
    sombreamento_variaveis();
    tipo_dados_inteiros();
    tipo_dados_ponto_flutuante();
    tipo_dados_char();
    tipo_dados_tuplas();
    tipo_dados_matriz();
}

fn dados_imutaveis() {

    // Dados Imutaveis
    let x = 10;
    let y = 20;
    let z = 30;

    println!("Eixo X, Y, Z = ({}, {}, {})", x, y, z);
}

fn dados_mutaveis() {

    // Dados mutaveis
    let mut a = 100;
    let mut b = 110;
    let mut c = 120;

    println!("Eixo A, B, C = ({}, {}, {})", a, b, c);

    a = a * 2 - 1;
    b = a * 2;
    c = a + b + c;

    println!("Eixo A, B, C = ({}, {}, {})", a, b, c);
}

fn constantes() {

    // Uso de contantes
    println!("Pontuação maxima: {}", PONTUACAO_MAXIMA);
}

fn sombreamento_variaveis() {

    // Sombreamento (Shadowing) de variaveis
    let codigo_usuario = 10_000;
    let codigo_usuario = codigo_usuario + 10;

    let espacos = "    ";
    let espacos = espacos.len(); 

    println!("Codigo Usuario: {}", codigo_usuario);
    println!("Espaços = {}", espacos);
}

fn tipo_dados_inteiros() {
    // Numero com sinal (Positivos/Negativos)
    let a: i8 = -127;
    let b: i16 = -200;
    let c: i32 = 100_000;
    let d: i64 = -200_000;

    println!("Número com sinal (Signed) = {}, {}, {}, {}", a, b, c, d);

    // Numero sem sinal (Positivos)
    let a: u8 = 127;
    let b: u16 = 200;
    let c: u32 = 100_000;
    let d: u64 = 200_000;

    println!("Número sem sinal (Unsigned) = {}, {}, {}, {}", a, b, c, d);
}

fn tipo_dados_ponto_flutuante() { 
    let a: f32 = 100.99;
    let b: f64 = -998.998;

    println!("a:f32 = {}, b:f64 = {}", a, b);
}

fn tipo_dados_char() {
    let a = 'A';
    let b = 'B';
    let emoticon = '😻';

    println!("a = {}, b = {}, emoticon = {}", a, b, emoticon);
}

fn tipo_dados_tuplas() {
    let tupla_a = (100_000, 'A', 1);
    let tupla_b = (100_000, 'B', 2);

    println!("Elementos da tupla A = ({}, {}, {})", tupla_a.0, tupla_a.1, tupla_a.2);
    println!("Elementos da tupla B = ({}, {}, {})", tupla_b.0, tupla_b.1, tupla_b.2);
}

fn tipo_dados_matriz() {
    let meses = [
        "Janeiro", 
        "Fevereiro", 
        "Março", 
        "Abril", 
        "Maio", 
        "Junho", 
        "Julho",
        "Agosto", 
        "Setembro", 
        "Outubro", 
        "Novembro", 
        "Dezembro"
    ];

    for (index, mes) in meses.iter().enumerate() {
        println!("{}/{}", index, mes);
    }
}
