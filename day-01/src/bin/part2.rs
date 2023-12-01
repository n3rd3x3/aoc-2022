fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {

    let input = input.split("\n\n");

    let mut sum: [u32; 3] = [0; 3];

    for line in input {
        let lines = line.lines();
        let mut elf_sum = 0;

        for line in lines {
            let line: u32 = line.trim().parse().unwrap();
            elf_sum += line;
        }
        sum.sort();
        if elf_sum > sum[0] {
            sum[0] = elf_sum;
        }
    }
    (sum[0] + sum[1] + sum[2]).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(include_str!("../test_input.txt"));
        assert_eq!(result, "45000");
    }
}