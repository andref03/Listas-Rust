# Estratégias de Operações com Listas em Rust

## Funcionalidades

O programa contém as seguintes funções principais:

### Função `executar_estrategia`

A função **`executar_estrategia`** permite executar uma estratégia em um vetor de inteiros. Ela recebe uma lista mutável e uma assinatura de uma função como argumentos, e executa essa função sobre o vetor fornecido.

#### Assinatura
```rust
fn executar_estrategia(lista: &mut Vec<i64>, estrategia: fn(&mut Vec<i64>));
```

### 1. `ordemCrescente` (Ordenação por Insertion Sort)
> Autor: Gustavo dos Santos Leão

A função **`ordemCrescente`** é responsável por ordenar um vetor de números inteiros em ordem crescente usando o algoritmo **Insertion Sort**. Esse algoritmo percorre o vetor e insere cada elemento na posição correta dentro da parte já ordenada.

**Assinatura:**
```rust
fn ordemCrescente(lista: &mut Vec<i64>)
```

### 2. ordena_decrescente (Ordenação por Insertion Sort)
> Autor: Heuller Ramos Vieira

A função **ordena_decrescente** é responsável por ordenar um vetor de números inteiros em ordem decrescente usando o algoritmo *Insertion Sort*. Esse algoritmo percorre o vetor e insere cada elemento na posição correta dentro da parte já ordenada.

**Assinatura:**
```rust
fn ordem_decrescente(lista: &mut Vec<i64>)
```

### 3. `removeDuplicatas`
> Autor: Bernardo Ruas Oliveira

A função **`removeDuplicatas`** implementa um algoritmo eficiente para remover elementos duplicados de um vetor (Vec<i64>), preservando a ordem da primeira aparição de cada elemento. Para garantir um desempenho otimizado, a função emprega um HashSet para rastrear os valores já vistos. A remoção dos itens é realizada através do método "retain", que avalia cada elemento e o mantém apenas se for a primeira vez que ele é encontrado.

**Assinatura:**
```rust
fn removeDuplicatas(lista: &mut Vec<i64>)
```

### 4. `filtrar_pares` 
> Autor: André Felipe de Oliveira Lopes

A função **`filtrar_pares`** recebe um vetor mutável de inteiros e filtra apenas os números pares. Especificamente, a função contém um loop, o qual permite a iteração por todos os elementos do vetor. Dentro deste loop, há uma condição que verifica se o resto da divisão do elemento atual por 2 resulta em zero. Caso sim, este elemento é adicionado no vetor de ´pares´. Neste caso, é possível perceber também que, se um elemento repetido aparecer, ele será novamente verificado e, se for par, será adicionado no vetor de ´pares´ da mesma forma. Ao término da iteração, um ponteiro do vetor principal recebe o vetor de ´pares´. Ou seja, o vetor original é atualizado. Como se trata de um ponteiro, a alteração é refletida em qualquer espaço do código, inclusive na função ´main´.

**Assinatura:**
```rust
fn filtrar_pares(lista: &mut Vec<i64>)
```
