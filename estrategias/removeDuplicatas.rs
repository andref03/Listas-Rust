///Remove todas as duplicatas de elementos em um vetor de int64 deixando apenas os
/// valores unicos e respeitando a ordem em que eles aparecem no vetor
/// também altera o tamanho do vetor o dimminuindo para não ficar espaços nulos
/// ou com lixo no vetor
/// 
/// Argumentos-> recebe um vetor de inteiros através de um ponteiro mut que faz com que 
/// seja possível alterar os valores e o tamanho do vetor
/// 
/// Exemplo:
/// 
/// let mut numeros = vec![1,2,3,1,3,5,2,1,3,4,]
/// removeDuplicatas(&mut numeros)
/// 
/// Saída : {1,2,3,5,4}


use std::collections::HashSet;//para remover duplicatas
fn removeDuplicatas(lista: &mut Vec<i64>) {
    // 1. Cria um HashSet para rastrear os elementos que já vimos.
    let mut vistos = HashSet::new();

    // 2. Usa o método `retain` para manter apenas os elementos únicos.
    lista.retain(|elemento| vistos.insert(*elemento));
}