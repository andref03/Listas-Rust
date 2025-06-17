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
