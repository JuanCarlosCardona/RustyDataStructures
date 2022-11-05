use crate::sorting_algorithms::{insertion_sort, selection_sort};

mod sorting_algorithms;

fn main() {
    let mut example: [i32; 10] = [10, 5, 8, 24, 32, 59, 60, 1, 45, 9];

    insertion_sort(example.as_mut());
    println!("{:?}", example);


    //let bubble_sorted_array: &mut[i32] = bubble_sort::bubble_sort(example.as_mut());
    //println!("{:?}", bubble_sorted_array)
}
