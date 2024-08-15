

*This is part of a process of learning the core principles of Rust by building common algorithms.*

# What is bubblesort?


You can think of bubble sort as a way of taking 2 values and put them in a private bubble, and then check which of those two values in the bubble is largest, and then switch them so the smaller one is first, and then move to the next 2 values, and do the same. Every time there is a switch needed, it is recorded. And if there was a switch during the bubble checks, the whole process is repeated.

**The reason why its called bubblesort is because you need to compare two values at a time, as if they are in a bubble. And then it moves to the next element. Think of bubbles moving up.**

# What can you sort with bubblesort?

You can sort any collection of values that can be ordered. In other words types that have the PartialOrder (Order) trait.


## Pseudocode

``` 
let changed = true; 

Start loop 

    Start Loop from first to last element of the collection
        If element < next element 
            do nothing
            changed = false
        else 
            swap the position of both elements
            // you need to know the position of the elements to swap
            changed = true
    End loop
    j
    if checked = false 
        break // stop the loop 

}

```

## How to iterate over a vec collection and get the index of the value?

```rust
let collection = vec![1,3,5];
for (index, element) in collection.enumerate() {
    println!("{} {}", index, element);
}
```

## How to swap 2 values in 2 positions in a vector?
```rust
let collection = vec![1,2];
assert_eq!(collection.swap(1,0), [2,1]);
```


## The problems faced when coding the bubblesort in rust for the first time

### 1. Understanding the borrowing rules of Rust. 

- You can have many immutable references at the same time (but you can't have any mutable references then!) 
- You can have only one immutable reference at any given time (but then you can't have any immutable references)

A solution is to iter_mut over the original collection, and mutate it. To have multiple immutable references, you can clone the original collection.

The algorithm:

Loop over all elements of the vector
Compare the current element to the next element in the vector
Swap if needed both elements
If there is a swap set the switched value to true
Stop looping when the switched value is false


### 2. When working with for loops on vectors and comparing indices, you need a way to know when you need to compare and when not. Because you will panic if you are trying to compare the last element with a non-existing next element.

That's why you need an if block which checks if you've reached the last element before you compare the current element with the next element.



# Extracting the bubblesort function for i32
This extraction can be found in the **extract_bubblesort_function** branch.

# Extracting a generic bubblesort function for Vec<T>
You will need to implement Display, Debug and maybe some other traits for the generic Vec. But you can't implement foreign traits or foreign types (remember the orphan rule). That's why you need to create a new local type.
 Then you can implement traits for it.



