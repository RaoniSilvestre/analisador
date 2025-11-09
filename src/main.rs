use analisador_lexico::arvore_sintatica::{Expressao, OperadorBinario, OperadorUnario};

fn main() {
    let cinco = Expressao::new_num(5);
    let dois = Expressao::new_num(2);
    let adicao = Expressao::new_bin_op(OperadorBinario::Adicao, cinco, dois);

    let sete = Expressao::new_num(7);
    let negacao = Expressao::new_un_op(OperadorUnario::Negacao, sete);

    let tres = Expressao::new_num(3);
    let pinguim = Expressao::new_un_op(OperadorUnario::Pinguinacao, tres);

    let multiplicacao = Expressao::new_bin_op(OperadorBinario::Multiplicacao, negacao, pinguim);

    let divisao_aninhada = Expressao::new_bin_op(OperadorBinario::Divisao, multiplicacao, adicao);

    let cinco_ex6 = Expressao::new_num(5);
    let vinte = Expressao::new_num(20);

    let negacao_vinte = Expressao::new_un_op(OperadorUnario::Negacao, vinte);

    let adicao_complexa = Expressao::new_bin_op(OperadorBinario::Adicao, cinco_ex6, negacao_vinte);

    let expressao_final = Expressao::new_bin_op(
        OperadorBinario::RestoDivisao,
        divisao_aninhada,
        adicao_complexa,
    );

    println!("Expressão Final Complexa: {}", expressao_final);
    println!("Resultado: {:?}", expressao_final.avaliar());

    expressao_final.imprimir_arvore();
}
