use std::collections::HashSet;

fn main() {
    println!("1 task response={}", get_priorities_sum("input"));
    println!("2 task response={}", get_priorities_sum("input"));
}

struct Symbol {
    char: char
}

impl Symbol {
    fn get_code(&self) -> u8 {
        let common_code = self.char as u8;
        if common_code > 96 {
            (common_code - 96) as u8
        } else {
            (common_code - 38) as u8
        }
    }
}

fn get_priorities_sum(file_path: &str) -> u64 {
    read_file(file_path)
        .lines()
        .map(|rucksack_items| {
            let mut left_part_items = HashSet::new();
            let half = rucksack_items.len() / 2;
            let common = rucksack_items.chars().enumerate().find(|(i, char)| {
                if *i < half {
                    left_part_items.insert(*char);
                } else if left_part_items.contains(char) {
                    return true;
                }

                false
            }).unwrap();

            Symbol{char: common.1}.get_code()
        })
        .fold(0, |acc, x| acc + x as u64)
}

fn read_file(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_1_test() {
        assert_eq!(get_priorities_sum("input-test"), 157)
    }

    #[test]
    fn solution_1() {
        assert_eq!(get_priorities_sum("input"), 8252)
    }
}
