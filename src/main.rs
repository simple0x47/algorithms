use crate::sort::insertion::{insertion_sort, reverted_insertion_sort};
use crate::sort::selection::selection_sort;

mod sort;

fn main() {
    println!("Hello, world!");

    let mut A = [10, 5, 2, 3, 1, 8];

    //insertion_sort(A.as_mut_slice());
    selection_sort(A.as_mut_slice());

    println!("Sort: ");
    for i in A {
        print!("{}", i);
    }
    println!();
}
