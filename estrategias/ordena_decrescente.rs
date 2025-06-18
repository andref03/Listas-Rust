/// Ordena uma lista de inteiros em ordem decrescente utilizando o algoritmo de inserção (Insertion Sort).
///
/// # Parâmetros
///
/// * `lista` - Um vetor mutável de números inteiros (`i64`) que será ordenado diretamente (in-place).
///
/// # Exemplo
///
/// ```
/// let mut numeros = vec![3, 1, 2];
/// ordena_decrescente(&mut numeros);
/// assert_eq!(numeros, vec![3, 2, 1]);
/// ```
///
/// # Complexidade
///
/// - Pior caso: O(n²) — ocorre quando a lista está em ordem crescente.
/// - Melhor caso: O(n) — ocorre quando a lista já está em ordem decrescente.
///
/// # Observações
///
/// Este é um algoritmo simples e eficiente apenas para listas pequenas ou quase ordenadas.

pub fn ordena_decrescente(lista: &mut Vec<i64>) {
    let n = lista.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && lista[j - 1] < lista[j] { // Alterado de '>' para '<' para ordenação decrescente
            lista.swap(j, j - 1);
            j -= 1;
        }
    }
}
