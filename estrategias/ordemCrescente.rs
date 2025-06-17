/// Ordena uma lista de inteiros em ordem crescente usando o algoritmo de inserção.
///
/// # Argumentos
///
/// * `lista` - Um vetor mutável de números inteiros (`i64`) que será ordenado in-place.
///
/// # Exemplo
///
/// ```
/// let mut numeros = vec![3, 1, 2];
/// ordemCrescente(&mut numeros);
/// ```
///
/// # Complexidade
///
/// - Pior caso: O(n²)
/// - Melhor caso (lista já ordenada): O(n)
///
/// # Observação
///
/// Este é um algoritmo simples, eficiente apenas para listas pequenas ou quase ordenadas.

pub fn ordemCrescente(lista: &mut Vec<i64>) {
    let n = lista.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && lista[j - 1] > lista[j] {
            lista.swap(j, j - 1);
            j -= 1;
        }

    }
}
