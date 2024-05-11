use std::cmp::Ordering;

fn build_heap<T: Ord>(arr: &mut [T], is_max_heap: bool) {
    let mut i = (arr.len() - 1) / 2;
    while i > 0 {
        heapify(arr, i, is_max_heap);
        i -= 1;
    }
    heapify(arr, 0, is_max_heap);
}
fn heapify<T: Ord>(arr: &mut [T], i: usize, is_max_heap: bool) {
    let mut comparator: fn(&T, &T) -> Ordering = |a, b| a.cmp(b);
    if !is_max_heap {
        comparator = |a, b| b.cmp(a);
    }
    let mut idx = i;
    let l = 2 * i + 1;
    let r = 2 * i + 2;

    if l < arr.len() && comparator(&arr[l], &arr[idx]) == Ordering::Greater {
        idx = l;
    }
    if r < arr.len() && comparator(&arr[r], &arr[idx]) == Ordering::Greater {
        idx = r;
    }
    if idx != i {
        arr.swap(i, idx);
        heapify(arr, idx, is_max_heap)
    }
}

pub fn heap_sort<T: Ord>(arr: &mut [T], ascending: bool) {
    if arr.len() <= 1 {
        return;
    }
    // Build heap based on the order
    build_heap(arr, ascending);
    let mut end = arr.len() - 1;
    while end > 0 {
        arr.swap(0, end);
        heapify(&mut arr[..end], 0, ascending);
        end -= 1;
    }
}
