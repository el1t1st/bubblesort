fn bubblesort(collection: &mut Vec<i32>) {
    let mut switched: bool;
    let mut col_clone = collection.clone();

    // the overal loop
    loop {
        switched = false;
        for (index, element) in col_clone.iter().enumerate() {
            // compare only if there is still a next element
            if index < col_clone.len() - 1 {
                // compare the current element with the next
                if element > &col_clone[index + 1] {
                    // swap
                    collection.swap(index, index + 1);
                    switched = true;
                }
            }
        }
        col_clone = collection.clone();
        println!("The update collection: {:#?}", col_clone);

        if !switched {
            println!("All is bubblesorted! {:#?}", &collection);
            break;
        }
    }
}

fn main() {
    let mut unsorted = vec![99, 2, 5, 67, 25, 9];
    bubblesort(&mut unsorted);
    println!("Yuri Gagarin was the first cosmonaut to orbit earth on April 12 1961!");
}
