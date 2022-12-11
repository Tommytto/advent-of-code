fn main() {
    let solution_1 = solve_part_1("input");
    let solution_2 = solve_part_2("input");

    dbg!(solution_1);
    dbg!(solution_2);
}

#[derive(Debug)]
struct Record(usize, usize);

impl TryFrom<&str> for Record {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bounds = value.split('-').collect::<Vec<&str>>();
        let left_bound = bounds.get(0).unwrap().parse().unwrap();
        let right_bound = bounds.get(1).unwrap().parse().unwrap();

        Ok(Record(left_bound, right_bound))
    }
}

fn solve_part_1(file_path: &str) -> usize {
    let mut counts = 0;
    for line in read_file(file_path).lines() {
        let mut parts = line.split(',');
        let (elf1, elf2) = (
            parts.next().unwrap(),
            parts.next().unwrap(),
        );

        let record_1 = Record::try_from(elf1).unwrap();
        let record_2 = Record::try_from(elf2).unwrap();

        let record_1_bigger = record_1.0 <= record_2.0 && record_1.1 >= record_2.1;
        let record_2_bigger = record_2.0 <= record_1.0 && record_2.1 >= record_1.1;
        if record_1_bigger || record_2_bigger {
            counts += 1;
        }
    }

    counts
}

fn solve_part_2(file_path: &str) -> usize {
    let mut counts = 0;
    for line in read_file(file_path).lines() {
        let mut parts = line.split(',');
        let (elf1, elf2) = (
            parts.next().unwrap(),
            parts.next().unwrap(),
        );

        let record_1 = Record::try_from(elf1).unwrap();
        let record_2 = Record::try_from(elf2).unwrap();

        if !(record_1.0 > record_2.1 || record_1.1 < record_2.0) {
            counts += 1;
        }
    }

    counts
}

fn read_file(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_part_1_test_input() {
        assert_eq!(solve_part_1("test-input"), 2)
    }

    #[test]
    fn solution_part_2_test_input() {
        assert_eq!(solve_part_2("test-input"), 4)
    }

    #[test]
    fn solution_part_1_normal_input() {
        assert_eq!(solve_part_1("input"), 498)
    }

    #[test]
    fn solution_part_2_normal_input() {
        assert_eq!(solve_part_1("input"), 498)
    }
}
