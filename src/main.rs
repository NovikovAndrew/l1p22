fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    let removed = remove_by_index(&mut vec, 1);
    let swap_removed = swap_remove_by_index(&mut vec, 2);
    println!("deleted: {}", removed); // 2
    println!("swap deleted: {}", swap_removed); // 4

    println!("vector is {:?}", vec); // [1, 3, 5]

    let new_vec = remove_with_iter(&vec, 1);
    println!("new vector is {:?}", new_vec); //  [1, 5]
}

// способ 1
// удаляет элемент по индексу
fn remove_by_index(vec: &mut Vec<i32>, index: usize) -> i32 {
    vec.remove(index)
}

// способ 2
// удаляет элемент по индексу, и заменяем удаленный элемент последним числом
fn swap_remove_by_index(vec: &mut Vec<i32>, index: usize) -> i32 {
    vec.swap_remove(index)
}

// способ 3
// создаем новый вектор, пропуская элемент с индексом i
fn remove_with_iter(vec: &Vec<i32>, index: usize) -> Vec<i32> {
    vec.iter()
        .enumerate()
        .filter_map(|(i, &v)| {
            if i == index {
                return None
            }

            Some(v)
        })
        .collect()
}




