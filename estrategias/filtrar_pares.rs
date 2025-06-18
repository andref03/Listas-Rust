pub fn filtrar_pares(vetor: &mut Vec<i64>) {
    let mut pares = Vec::new();

    for valor in vetor.iter() {
        if valor % 2 == 0 {
            pares.push(*valor);
        } 

    }
    *vetor = pares;
}
