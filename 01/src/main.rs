use std::cmp;
use std::fs;
use std::iter::zip;

fn main() {
    let input = fs::read_to_string("data").expect("Missing data file");
    let result = calculate_from_string(&input);
    println!("{result}");
    return;
}

fn calculate_from_string(input: &str) -> u32 {
    let mut a = vec![];
    let mut b = vec![];
    for line in input.split("\n") {
        let mut item_iter = line.split("   ");
        a.push(item_iter.next().unwrap().parse().unwrap());
        b.push(item_iter.next().unwrap().parse().unwrap());
    }
    return sum_pairs(
        &pair_arrays( &a, &b )
    );
}

fn pair_arrays(a: &Vec<u32>, b: &Vec<u32>) -> Vec<(u32, u32)> {
    let mut a2 = a.clone();
    let mut b2 = b.clone();

    a2.sort();
    b2.sort();

    let paired = zip(
        a2.iter().copied(),
        b2.iter().copied()
    ).collect();
    return paired;
}

fn sum_pairs(paired: &Vec<(u32, u32)>) -> u32 {
    let differences = paired
        .iter()
        .map(
            |(a, b)| cmp::max(a, b) - cmp::min(a, b)
        )
        .sum();
    return differences;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_pairing() {
        let a = vec![1,2,3];
        let b = vec![3,4,1];
        assert_eq!(pair_arrays(&a, &b), vec![(1, 1), (2, 3), (3, 4)]);
    }

    #[test]
    fn test_sum() {
        let a = vec![1,2,3];
        let b = vec![3,4,1];
        assert_eq!(sum_pairs(&pair_arrays(&a, &b)), 2);
    }

    #[test]
    fn test_sample_input() {
        let sample = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(
            calculate_from_string(&sample), 11
        );
    }

}