// We need to add restrictions to the generic type for bubblesort
// We need to implement the Debug trait for T

fn bubblesort<T>(collection: &mut [T]) {
    let mut switched: bool;
    let col_clone = collection.clone();

    loop {
        switched = false;
        for (index, element) in col_clone.iter().enumerate() {
            if index < col_clone.len() {
                // perform the compare of current with next
                if element > &col_clone[index + 1] {
                    switched = true;
                    collection.swap(index, index + 1);
                }
            }
        }
        if !switched {
            break;
        }
    }
}

fn main() {
    let mut col_to_sort = vec![9, 2, 1, 99, 5];
    bubblesort(&mut col_to_sort);
    todo!();
}
