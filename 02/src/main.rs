use std::fs;

fn main() {
    let input = fs::read_to_string("data").expect("Missing data file");
    let result = calculate_from_string(&input);
    println!("{result}");
    return;
}

fn calculate_from_string(input: &str) -> u32 {
    let mut num_safe = 0;
    for line in input.split("\n") {
        let item_iter: Vec<i32> = line.split(" ").map(| x | x.parse().unwrap()).collect();
        if is_safe(item_iter) {
            num_safe += 1;
        }
    };
    return num_safe;
}

fn is_safe(readings: Vec<i32>) -> bool {
    let differences: Vec<i32> = readings.windows(2).map(| a | a[1]-a[0]).collect();
    println!("{:?}", differences);
    if differences.iter().all(|&x| {-4 < x && x < 0}) {
        return true;
    }
    else if differences.iter().all(|&x| {0 < x && x < 4}) {
        return true;
    }
    else {
        return false;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_increasing() {
        let sample = vec![1, 3, 6, 7, 9];
        assert_eq!(
            is_safe(sample), true
        );
    }

    #[test]
    fn test_unsafe_increasing() {
        let sample = vec![1, 2, 7, 8, 9];
        assert_eq!(
            is_safe(sample), false
        );
    }


    #[test]
    fn test_safe_decreasing() {
        let sample = vec![7, 6, 4, 2, 1];
        assert_eq!(
            is_safe(sample), true
        );
    }

    #[test]
    fn test_unsafe_decreasing() {
        let sample = vec![9, 7, 6, 2, 1];
        assert_eq!(
            is_safe(sample), false
        );
    }

    #[test]
    fn test_unsafe_mixed() {
        let sample = vec![1, 3, 2, 4, 5];
        assert_eq!(
            is_safe(sample), false
        );
    }

    #[test]
    fn test_unsafe_equal() {
        let sample = vec![8, 6, 4, 4, 1];
        assert_eq!(
            is_safe(sample), false
        );
    }


    #[test]
    fn test_sample_input() {
        let sample = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(
            calculate_from_string(&sample), 2
        );
    }

}