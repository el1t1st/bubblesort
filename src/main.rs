// The reason why I create a clone of the collection, is because of the borrowing rules of Rust.
// You can only have one mutable reference at any given time. or you can have many immutable
// references. So the immugable references are used for looping and copamaring, and the original
// collection is mutated as the bublesort process runs.
//
fn main() {
    let mut v = vec![99, 9, 18, 2, 15, 66, 100];
    let mut vcopy = v.clone();
    let mut changed: bool;

    loop {
        changed = false;
        for (index, element) in vcopy.iter().enumerate() {
            // println!("The element {} on index {}", element, index);
            if index < &vcopy.len() - 1 {
                println!("Still a next element : {} {}", &vcopy[index + 1], element);
                if element > &vcopy[index + 1] {
                    println!("need to swap");
                    v.swap(index, index + 1);
                    changed = true;
                }
            }
        }

        println!("the updated vec: {:#?}", &v);
        vcopy = v.clone();

        if !changed {
            println!("The collection is sorted using bubblesort: {:#?}", v);
            break;
        }
    }
}
