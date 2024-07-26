pub fn execute_part_a() {
    let mut a = "456".to_string();
    let b = a.split_off(1);
}

fn solve_a(input: &str) -> i32 {
    let input_list: Vec<&str> = input.lines().collect();

    input_list.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"1abc2
      pqr3stu8vwx
      a1b2c3d4e5f
      treb7uchet";

    #[test]
    fn solves_example_a() {
        let result = solve_a(INPUT);
        assert_eq!(result, 142);
    }
}
