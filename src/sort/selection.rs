pub fn selection_sort(A: &mut [i32])  {
    for i in 0..A.len() - 1 {
        let min_index: usize = find_smallest_after(A, i);

        A.swap(i, min_index);
    }
}

fn find_smallest_after(A: &[i32], after: usize) -> usize {
    let mut min: i32 = A[after];
    let mut min_index: usize = after;

    for i in after..A.len() {
        if A[i] < min {
            min = A[i];
            min_index = i;
        }
    }

    return min_index;
}