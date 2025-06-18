/// Filtra apenas os números pares do vetor de inteiros
/// 
/// Esta função percorre todos os elementos do vetor original e mantém
/// apenas aqueles que são pares (divisíveis por 2). O vetor original é
/// substituído pelo novo vetor contendo apenas os pares.
/// 
/// # Argumentos
///
/// * `vetor` - Um vetor mutável de números inteiros (`<i64>`) que será filtrado.
///
/// # Exemplo
///
/// ```
/// let mut numeros = vec![1, 2, 3, 4, 5, 6];
/// filtrar_pares(&mut numeros);
/// assert_eq!(numeros, vec![2, 4, 6]);
/// ```
///
/// # Observação
///
/// O vetor original é modificado. Os elementos ímpares são removidos.
///
/// # Complexidade
///
/// - Tempo: O(n), onde n é o tamanho do vetor.
/// - Espaço adicional: O(n), para armazenar os pares temporariamente.

pub fn filtrar_pares(vetor: &mut Vec<i64>) {
    let mut pares = Vec::new();

    for valor in vetor.iter() {
        if valor % 2 == 0 {
            pares.push(*valor);
        } 

    }
    *vetor = pares;
}
