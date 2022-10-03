pub(crate) fn bubble_sort(array: &mut [i32]) -> &mut [i32]{

    let sorted_array: &mut[i32] = array;

    for i in 0..sorted_array.len() - 1{

        for j in 0..sorted_array.len() - 1- i{

            if sorted_array[j] > sorted_array[j + 1]{
                sorted_array.swap(j, j + 1);
            }
        }

    }
    return sorted_array
}