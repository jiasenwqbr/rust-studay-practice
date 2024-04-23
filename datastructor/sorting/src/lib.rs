pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    for _ in 0..v.len() {
        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        let mut v = vec![4, 5, 3, 2, 1];
        bubble_sort(&mut v);
        assert_eq!(vec![1, 2, 3, 4, 5], v);
    }
}
