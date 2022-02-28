pub fn insertion_sort(slice: &mut [i32]) {
    for j in 1..slice.len() {
        let key = slice[j];

        for i in (0..j).rev() {
            if slice[i] > key {
                slice.swap(i + 1, i);
            } else {
                break;
            }
        }
    }
}

pub fn reverted_insertion_sort(slice: &mut [i32]) {
    for j in 1..slice.len() {
        let key = slice[j];

        for i in (0..j).rev() {
            if slice[i] < key {
                slice.swap(i + 1, i);
            } else {
                break;
            }
        }
    }
}