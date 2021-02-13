fn fill_rest<'a, T: Copy + PartialOrd>(target: &mut Vec<T>, source: &'a [T], index: usize) {
    for val in source[index..].iter() {
        target.push(val.to_owned());
    }
}

fn merge<'a, T: Copy + PartialOrd>(left: &'a [T], right: &'a [T]) -> Vec<T> {
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut merged = Vec::new();

    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] < right[right_idx] {
            merged.push(left[left_idx].to_owned());
            left_idx += 1;
        } else {
            merged.push(right[right_idx].to_owned());
            right_idx += 1;
        }
    }
    // at this point possible that there are left overs on one side
    if left_idx < left.len() {
        fill_rest(&mut merged, left, left_idx);
    } else {
        fill_rest(&mut merged, right, right_idx);
    }
    merged
}

pub fn mergesort<T: Copy + PartialOrd>(list: &mut [T]) {
    let length = list.len();
    if length <= 1 {
        return;
    }
    let pivot = length / 2;
    // sort left
    mergesort(&mut list[0..pivot]);
    // sort right
    mergesort(&mut list[pivot..]);
    // memcpy the merged vector into our starting list
    list.copy_from_slice(&merge(&list[0..pivot], &list[pivot..]))
}

#[cfg(test)]
mod tests {
    use super::{merge, mergesort};
    #[test]
    fn it_merges() {
        assert_eq!(merge::<usize>(&vec![], &vec![]), vec![]);
        assert_eq!(merge(&[1, 2, 3], &vec![4, 5, 6]), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(merge(&[1, 3], &vec![2, 4]), vec![1, 2, 3, 4]);
        assert_eq!(merge(&[1, 3], &vec![2]), vec![1, 2, 3]);
        assert_eq!(merge(&[1,], &vec![-1, 10]), vec![-1, 1, 10]);
        assert_eq!(merge(&["a",], &vec![]), vec!["a"]);
        assert_eq!(merge(&["a", "c"], &vec!["b"]), vec!["a", "b", "c"]);
    }
    #[test]
    fn it_mergesorts() {
        let mut arr = vec![];
        mergesort::<usize>(&mut arr);
        assert_eq!(arr, vec![]);

        let mut arr = vec![-2, -1];
        mergesort::<i8>(&mut arr);
        assert_eq!(arr, vec![-2, -1]);

        let mut arr = vec![5, -2, -1];
        mergesort::<i8>(&mut arr);
        assert_eq!(arr, vec![-2, -1, 5]);

        let mut arr = vec![1, 5, 6, 3];
        mergesort::<usize>(&mut arr);
        assert_eq!(arr, vec![1, 3, 5, 6]);

        let mut arr = vec![100, -1, 99];
        mergesort::<i32>(&mut arr);
        assert_eq!(arr, vec![-1, 99, 100]);

        let mut arr = vec![6, 5, 4, 3, 2, 1];
        mergesort::<i32>(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);

        let mut arr = vec!["Zed", "Aaron", "Jimmy", "Billy"];
        mergesort(&mut arr);
        assert_eq!(arr, vec!["Aaron", "Billy", "Jimmy", "Zed"]);
    }
}
