pub fn bucket_sort(arr: &[usize]) -> Vec<usize> {
    if arr.is_empty() {
        return vec![];
    }

    let max = *arr.iter().max().unwrap();
    let len = arr.len();
    let mut buckets = vec![vec![]; len + 1];
    for x in arr {
        buckets[len * *x / max].push(*x);
    }
    for bucket in buckets.iter_mut() {
        insertion_sort(bucket);
    }
    let mut result = vec![];
    for bucket in buckets {
        for x in bucket {
            result.push(x);
        }
    }
    result
}
pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        let cur = arr[i];
        while j > 0 && cur < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = cur;
    }
}
#[cfg(test)]
mod tests {
    use super::bucket_sort;

    #[test]
    fn descending() {
        let mut v = vec![
            47, 71, 96, 94, 10, 74, 44, 49, 85, 36, 53, 56, 47, 25, 27, 46, 45, 47, 76, 48, 85, 82,
            4, 79, 66, 64, 38, 9, 31, 90, 18, 72, 88, 85, 44, 30, 49, 93, 58, 82, 13, 24, 46, 94,
            68, 82, 14, 28, 43, 81, 46, 27, 42, 44, 23, 62, 88, 51, 43, 98, 24, 65, 45, 60, 80, 29,
            15, 33, 24, 28, 28, 22, 13, 67, 41, 13, 66, 66, 75, 16, 80, 68, 10, 20, 25, 28, 2, 85,
            32, 32, 46, 27, 88, 51, 40, 51, 39, 82, 47, 24, 68, 42, 1, 97, 50, 67, 77, 52, 100, 5,
            45, 25, 57, 72, 73, 99, 91, 86, 76, 90, 33, 48, 89, 77, 50, 81, 98, 54, 56, 52, 97, 71,
            25, 80, 2, 10, 34, 57, 90, 73, 51, 54, 46, 7, 29, 76, 36, 18, 72, 76, 73, 69, 57, 22,
            99, 59, 67, 38, 5, 25, 63, 61, 9, 39, 95, 16, 22, 25, 32, 18, 23, 39, 81, 80, 100, 70,
            63, 9, 35, 70, 88, 91, 98, 30, 9, 38, 53, 91, 19, 27, 100, 51, 7, 47, 92, 35, 54, 35,
            70, 52, 3, 68, 51, 41, 18, 55, 27, 41, 98, 50, 97, 59,
        ];
        let v1 = bucket_sort(&mut v);
        println!("{:?}", v);
        assert_eq!(
            vec![
                1, 2, 2, 3, 4, 5, 5, 7, 7, 9, 9, 9, 9, 10, 10, 10, 13, 13, 13, 14, 15, 16, 16, 18,
                18, 18, 18, 19, 20, 22, 22, 22, 23, 23, 24, 24, 24, 24, 25, 25, 25, 25, 25, 25, 27,
                27, 27, 27, 27, 28, 28, 28, 28, 29, 29, 30, 30, 31, 32, 32, 32, 33, 33, 34, 35, 35,
                35, 36, 36, 38, 38, 38, 39, 39, 39, 40, 41, 41, 41, 42, 42, 43, 43, 44, 44, 44, 45,
                45, 45, 46, 46, 46, 46, 46, 47, 47, 47, 47, 47, 48, 48, 49, 49, 50, 50, 50, 51, 51,
                51, 51, 51, 51, 52, 52, 52, 53, 53, 54, 54, 54, 55, 56, 56, 57, 57, 57, 58, 59, 59,
                60, 61, 62, 63, 63, 64, 65, 66, 66, 66, 67, 67, 67, 68, 68, 68, 68, 69, 70, 70, 70,
                71, 71, 72, 72, 72, 73, 73, 73, 74, 75, 76, 76, 76, 76, 77, 77, 79, 80, 80, 80, 80,
                81, 81, 81, 82, 82, 82, 82, 85, 85, 85, 85, 86, 88, 88, 88, 88, 89, 90, 90, 90, 91,
                91, 91, 92, 93, 94, 94, 95, 96, 97, 97, 97, 98, 98, 98, 98, 99, 99, 100, 100, 100
            ],
            v1
        );
    }
}
