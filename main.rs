mod estrategias;

fn executar_estrategia(lista: &mut Vec<i64>, estrategia: fn(&mut Vec<i64>)) {
    estrategia(lista);
    println!("Resultado: {:?}", lista);
}

fn main() {
    let mut numeros : Vec<i64> = vec![5, 5, 1, 0, 9, 8, 6, 2, 8, 5, 2, 9, 1, 0, 4, 4];
    println!("Lista original: {:?}", numeros);

    executar_estrategia(&mut numeros, estrategias::ordemCrescente::ordemCrescente);

    executar_estrategia(&mut numeros, estrategias::removeDuplicatas::removeDuplicatas);
    
}
