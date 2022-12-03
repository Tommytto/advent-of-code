use std::env;
use std::fs;

fn main() -> Result<(), &'static str> {
    let file_name = env::args().nth(1).expect("File name with input should be provided");
    let contents = fs::read_to_string(file_name).expect("Provide path from cargo to input");

    let vectors = string_to_vectors(contents);
    println!("1 part. Maximum calories: {}", get_max_sum_top_n(&vectors, 1).unwrap());
    println!("2 part. Maximum calories for top3: {}", get_max_sum_top_n(&vectors, 3).unwrap());

    Ok(())
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

fn get_max_sum_top_n(elves_items: &Vec<Vec<usize>>, n: usize) -> Result<usize, &str> {
    if elves_items.len() < n {
        return Err("Count of elves is too small");
    }
    let mut sums: Vec<usize> = vec![];

    for x in elves_items {
        sums.push(x.iter().sum());
    }

    sums.sort();

    Ok(sums.get(sums.len()-n..).unwrap().iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_maximum_positive() {
        let items = vec![vec![1,2], vec![3,4]];
        assert_eq!(get_max_sum_top_n(&items, 1), Ok(7));
    }

    #[test]
    fn return_0_on_empty() {
        assert_eq!(get_max_sum_top_n(&vec![], 1), Err("Count of elves is too small"));
    }

    #[test]
    fn calculate_maximum_3_works() {
        let items = vec![vec![1,2], vec![3,4], vec![10, 20], vec![10, 10]];
        assert_eq!(get_max_sum_top_n(&items, 3), Ok(57));
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
