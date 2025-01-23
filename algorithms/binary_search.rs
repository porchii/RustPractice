fn binary_search(arr: &Vec<i32>, x: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut l = 0;
    let mut r = arr.len();
    while r - l > 1 {
        let m = (l + r) / 2;

        if arr[m] <= x {
            l = m;
        } else {
            r = m;
        }
    }

    if arr[l] == x {
        Some(l + 1)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::binary_search;

    #[test]
    fn elemnt_exists() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(binary_search(&arr, 5), Some(5));
    }

    #[test]
    fn elemnt_not_exists() {
        let arr: Vec<i32> = vec![1, 2, 3, 5, 6, 7, 8, 9];
        assert_eq!(binary_search(&arr, 4), None);
    }

    #[test]
    fn array_is_empty() {
        let arr: Vec<i32> = vec![];
        assert_eq!(binary_search(&arr, 5), None);
    }
}
