mod bubble_sort;

fn main() {
    let mut example: [i32; 10] = [10, 5, 8, 24, 32, 59, 60, 1, 45, 9];

    let sorted_array: &mut[i32] = bubble_sort::bubble_sort(example.as_mut());

    println!("{:?}", sorted_array)
}
