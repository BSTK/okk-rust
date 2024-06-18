fn main() {
    println!("---- Funções -----");
    exibir_descricao_completa("Adamastor", 98, "Marceneiro");
    exibir_descricao_completa("Julia", 27, "Enfermeira");
    exibir_descricao_completa("Manoel", 50, "Motorista");
    exibir_descricao_completa("Maria", 68, "Dona de casa");

    println!("--------------------------------------------------------------------");
    declaracao_expressooes();
}

fn exibir_descricao_completa(
    usuario: &str, 
    idade: u16, 
    profissao: &str) {
    println!("Usuario: {}, Idade: {}, Profissão: {}", usuario, idade, profissao);
}

fn declaracao_expressooes() {
    let x = 100;
    let y = 200;
    let z = {
        let xy = x * y;
        if xy > x {
            xy + x
        } else {
            0
        }
    };

    println!("X, Y, Z = {}, {}, {}", x, y, z);
}