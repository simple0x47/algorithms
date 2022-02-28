pub fn merge_sort(A: &mut [i32]) {
    sort(A, 0, A.len() - 1);
}

fn sort(A: &mut [i32], start: usize, end: usize) {
    if end - start > 2 {
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

    for i in start..(middle + 1) {
        let j = middle + 1;

        if j > end {
            break;
        }

        if A[i] > A[j] {
            push_until(A, i, j, A[j]);
        }
    }
}

fn push_until(A: &mut [i32], to: usize, until: usize, value: i32) {
    for i in (pos..(until + 1)).rev() {
        A[i] = A[i - 1];
    }

    A[to] = value;
}