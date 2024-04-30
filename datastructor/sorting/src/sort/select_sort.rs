fn select_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        select_sort(&mut v);
        println!("{:?}", v);
        assert_eq!(vec![1, 3, 4, 6, 8, 11, 13], v);
    }
}
