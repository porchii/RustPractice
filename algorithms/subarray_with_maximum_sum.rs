fn main() {
    println!("20:01:27");
}

fn find_sum(arr: &Vec<i32>) -> Option<(i32, usize, usize)> {
    if arr.len() == 0 {
        return None;
    }

    let mut l = 0;
    let mut r = 0;
    let mut mn = 0;
    let mut mx: i32 = -1000000000;
    let mut sum = 0;
    let mut cur_l = 0;
    for i in 0..arr.len() {
        sum += arr[i];

        if sum - mn > mx {
            mx = sum - mn;
            l = cur_l;
            r = i + 1;
        }

        if mn > sum {
            mn = sum;
            cur_l = i + 1;
        }
    }

    Some((mx, l + 1, r))
}

#[cfg(test)]
mod tests {
    use super::find_sum;

    #[test]
    fn positive_number_in_vector() {
        let arr: Vec<i32> = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec();
        assert_eq!(find_sum(&arr), Some((55, 1, 10)));
    }
    #[test]
    fn negative_number_in_vector() {
        let arr: Vec<i32> = [-1, -2, -3, -4, -5, -6, -7, -8, -9, -10].to_vec();
        assert_eq!(find_sum(&arr), Some((-1, 1, 1)));
    }
    #[test]
    fn all_numbers() {
        let arr: Vec<i32> = [-1, 2, 3, -4, -5, 6, -7, -8, -9, -10].to_vec();
        assert_eq!(find_sum(&arr), Some((6, 6, 6)));
    }

    #[test]
    fn all_numbers2() {
        let arr: Vec<i32> = [1, -2, -3, 4, 5, 6, -7, 8, 9, 10].to_vec();
        assert_eq!(find_sum(&arr), Some((35, 4, 10)));
    }

    #[test]
    fn one_numver() {
        let arr: Vec<i32> = [1].to_vec();
        assert_eq!(find_sum(&arr), Some((1, 1, 1)));
    }

    #[test]
    fn all_zeroes() {
        let arr: Vec<i32> = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0].to_vec();
        assert_eq!(find_sum(&arr), Some((0, 1, 1)));
    }

    #[test]

    fn empty() {
        let arr: Vec<i32> = [].to_vec();
        assert_eq!(find_sum(&arr), None);
    }
}
