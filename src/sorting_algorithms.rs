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

pub(crate) fn selection_sort(array: &mut[i32]) -> &mut[i32]{

    let sorted_array: &mut[i32] = array;

    for i in 0..sorted_array.len(){

        let mut current_min: i32 = sorted_array[i];
        let mut min_value_index: usize = i;

        let mut j = i + 1;

        while j < sorted_array.len() {

            if current_min > sorted_array[j] {
                current_min = sorted_array[j];
                min_value_index = j;
            }

            j += 1;
        }
        sorted_array.swap(i, min_value_index);
    }

    sorted_array
}

pub(crate) fn insertion_sort(array: &mut[i32]){

    let mut j;
    
    for i in 0..array.len() {

        j = i;

        while j > 0  && array[j - 1] > array[j] {

            array.swap(j, j- 1);

            j -= 1;
        }

    }
}

