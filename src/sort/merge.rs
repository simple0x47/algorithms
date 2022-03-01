use std::cmp::max;

pub fn merge_sort(A: &mut [i32]) {
    sort(A, 0, A.len() - 1);
}

fn sort(A: &mut [i32], start: usize, end: usize) {
    if end - start > 1 {
        let middle: usize = (start + end) / 2;

        sort(A, start, middle);
        sort(A, middle + 1, end);
        merge(A, start, middle, end);
        return;
    }

    if A[end] < A[start] {
        A.swap(end, start);
    }
}

fn merge(A: &mut [i32], start: usize, middle: usize, end: usize) {
    if A[middle] <= A[middle + 1] {
        return;
    }

    let left = &A[start..(middle + 1)].to_owned();
    let right =  &A[(middle + 1)..(end + 1)].to_owned();

    let mut left_index: usize = 0;
    let mut right_index: usize = 0;

    for i in start..(end + 1) {
        let mut value = 0;

        if (right_index >= right.len()) ||
            ((left_index < left.len()) && (left[left_index] <= right[right_index])) {
            value = left[left_index];
            left_index += 1;
        } else {
            value = right[right_index];
            right_index += 1;
        }

        A[i] = value;
    }
}