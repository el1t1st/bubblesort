// This is an example of how to implement bubblesort for Vec<T>
//
use std::fmt;

fn bubble_sort<T>(collection: &mut Vec<T>)
where
    T: std::cmp::PartialOrd + std::clone::Clone + fmt::Display,
{
    struct Displayable<T>(Vec<T>);

    impl<T: fmt::Display> fmt::Display for Displayable<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let vec_to_display = &self.0;
            write!(f, "[")?;
            for (index, element) in vec_to_display.iter().enumerate() {
                if index != 0 {
                    write!(f, " ,")?;
                }
                write!(f, "{}", element)?;
            }
            write!(f, "]")
        }
    }

    let mut switched: bool;
    let mut collection_clone = collection.clone();

    loop {
        switched = false;

        for (index, element) in collection_clone.iter().enumerate() {
            if index < &collection_clone.len() - 1 {
                if element > &collection_clone[index + 1] {
                    switched = true;
                    collection.swap(index, index + 1);
                    println!(
                        "The swapped collection: {}",
                        Displayable(collection.to_vec())
                    )
                } else {
                    println!("Do nothing");
                }
            }
        }

        // copy the updated collection to the collection_clone, so you can rerun the sorting on the
        // updated collection
        collection_clone = collection.to_vec();

        // Once there are no swaps, meaning the switched is set to false, the collection is sorted
        if !switched {
            println!("Finished Sorting: {}", Displayable(collection.to_vec()));
            break;
        }
    }
}

fn main() {
    let mut vec_of_numbers: Vec<i32> = vec![1, 2, 99, 5, 72];
    vec_of_numbers.push(0);

    let mut vec_of_strings: Vec<String> = vec!["Dmitry Mendeleyev".to_string()];
    vec_of_strings.push("Yuri Gagarin".to_string());

    bubble_sort(&mut vec_of_numbers);
    bubble_sort(&mut vec_of_strings);
}
