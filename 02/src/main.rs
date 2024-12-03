use std::env;
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
        let to_iter;
        if env::args().filter(| arg | arg == "-2").count() > 0 {
            to_iter = dampened_vectors(&item_iter);
        } else {
            to_iter = vec![item_iter];
        }
        for sub_iter in to_iter {
            if is_safe(&sub_iter) {
                num_safe += 1;
                break;
            }
        }
    };
    return num_safe;
}

fn dampened_vectors(readings: &[i32]) -> Vec<Vec<i32>> {
    let mut modified = vec![];
    for i in 0..readings.len() {
        modified.push(
            readings.iter().cloned().enumerate().filter( |(j, _x)| *j != i).map( |(_j, x)| x).collect()
        );
    }
    return modified;
}

fn is_safe(readings: &[i32]) -> bool {
    let differences: Vec<i32> = readings.windows(2).map(| a | a[1]-a[0]).collect();
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
            is_safe(&sample), true
        );
    }

    #[test]
    fn test_unsafe_increasing() {
        let sample = vec![1, 2, 7, 8, 9];
        assert_eq!(
            is_safe(&sample), false
        );
    }


    #[test]
    fn test_safe_decreasing() {
        let sample = vec![7, 6, 4, 2, 1];
        assert_eq!(
            is_safe(&sample), true
        );
    }

    #[test]
    fn test_unsafe_decreasing() {
        let sample = vec![9, 7, 6, 2, 1];
        assert_eq!(
            is_safe(&sample), false
        );
    }

    #[test]
    fn test_unsafe_mixed() {
        let sample = vec![1, 3, 2, 4, 5];
        assert_eq!(
            is_safe(&sample), false
        );
    }

    #[test]
    fn test_unsafe_equal() {
        let sample = vec![8, 6, 4, 4, 1];
        assert_eq!(
            is_safe(&sample), false
        );
    }

    #[test]
    fn test_dampened_vectors() {
        let sample = vec![8, 6, 4, 4, 1];
        assert_eq!(
            dampened_vectors(&sample),
            vec![
                vec![6, 4, 4, 1],
                vec![8, 4, 4, 1],
                vec![8, 6, 4, 1],
                vec![8, 6, 4, 1],
                vec![8, 6, 4, 4]
            ]
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