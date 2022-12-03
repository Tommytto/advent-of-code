use std::env;
use std::fs;

fn main() {
    let file_name = env::args().nth(1).expect("File name with input should be provided");
    let contents = fs::read_to_string(file_name).expect("Provide path from cargo to input");

    let vectors = string_to_vectors(contents);
    println!("1 part. Maximum calories: {}", get_max_sum(&vectors));
    println!("2 part. Maximum calories for top3: {}", get_max_sum_3(&vectors));
}

fn string_to_vectors(str: String) -> Vec<Vec<usize>> {
    let mut result = vec![];
    result.push(vec![]);

    for line in str.lines() {
        if line.is_empty() {
            result.push(vec![]);
        } else {
            let num = line.parse::<usize>().unwrap();
            result.last_mut().unwrap().push(num);
        }
    }

    result
}

fn get_max_sum(elves_items: &Vec<Vec<usize>>) -> usize {
    let mut max: usize = 0;

    for item in elves_items.iter() {
        let sum = item.iter().fold(0, |sum, x| sum + x);

        if sum > max {
            max = sum;
        }
    }

    max
}

fn get_max_sum_3(elves_items: &Vec<Vec<usize>>) -> usize {
    let mut sums: Vec<usize> = vec![];

    for x in elves_items {
        sums.push(x.iter().sum());
    }

    sums.sort();

    sums.get(sums.len()-3..).unwrap().iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_maximum_works() {
        let items = vec![vec![1,2], vec![3,4]];
        assert_eq!(get_max_sum(&items), 7);
    }

    #[test]
    fn return_0_on_empty() {
        let items = vec![];
        assert_eq!(get_max_sum(&items), 0);
    }

    #[test]
    fn calculate_maximum_3_works() {
        let items = vec![vec![1,2], vec![3,4], vec![10, 20], vec![10, 10]];
        assert_eq!(get_max_sum_3(&items), 57);
    }

    #[test]
    fn parse_string() {
        let mut str = "".to_string();
        str.push_str("100\n");
        str.push_str("200\n");
        str.push_str("300\n");
        str.push_str("\n");
        str.push_str("200");
        assert_eq!(string_to_vectors(str), vec![vec![100, 200, 300], vec![200]])
    }
}
