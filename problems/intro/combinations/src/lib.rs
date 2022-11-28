#![forbid(unsafe_code)]

/// # Описание
/// Служебная функция, которая рекурсивно подсчитывает комбинацию из n элементов в rest
///
/// # Arguments
///
/// * `sofar`: Та часть, которая уже есть в комбинации
/// * `rest`: Оставшаяся часть от массива
/// * `n`: Сколько чисел осталось
///
/// returns: Vec<Vec<i32, Global>, Global>
///
fn comb(sofar: Vec<i32>, rest: &[i32], n: usize, result: &mut Vec<Vec<i32>>) {
    if n == 0 {
        result.push(sofar);
    } else {
        for i in 0..rest.len() {
            let mut new_sofar = sofar.clone();
            new_sofar.push(rest[i]);
            comb(new_sofar, &rest[i + 1..], n - 1, result);
        }
    }
}

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    comb(Vec::new(), arr, k, &mut result);

    result
}
