use std::{fs, io};
pub fn solve_input() -> io::Result<()> {
    let output = parse_input("input.txt")?;
    let mut sum = 0;
    for (a, b) in output {
        for i in a..=b {
            if is_invalid2(i) {
                // change this to is_invalid2 for the part 2
                sum += i;
            }
        }
    }
    println!("The sum is {sum}");
    Ok(())
}
fn is_invalid2(num: u64) -> bool {
    let len = (num as f64).log10().floor() as u16 + 1;
    let first_digit = num % 10;
    let mut part = 0;
    for i in 0..len {
        if i == len - 1 {
            break;
        }
        let c: u32 = (len - i - 1).into();
        let digit = num / 10_u64.pow(c) % 10;

        part = part * 10 + digit;

        if digit != first_digit && len % (i as u16 + 1) != 0 {
            continue;
        }
        let repeat = len / (i as u16 + 1);

        let mut number: u64 = 0;
        for _ in 0..repeat {
            number = number * 10_u64.pow(i as u32 + 1) + part
        }
        if number == num {
            return true;
        }
    }
    false
}

#[allow(unused)]
fn is_invalid(num: u64) -> bool {
    let len = (num as f64).log10().floor() as u16 + 1;
    if len % 2 == 1 {
        return false;
    }
    let mut i: u16 = 0;
    let mut j: u16 = len >> 1;
    while j < len {
        let a = num.div_euclid(10_u64.pow(i.into())) % 10;
        let b = num.div_euclid(10_u64.pow(j.into())) % 10;

        if a != b {
            return false;
        }
        i += 1;
        j += 1;
    }
    true
}
fn parse_input(filename: &str) -> io::Result<Vec<(u64, u64)>> {
    let content = fs::read_to_string(filename)?;
    let mut output: Vec<(u64, u64)> = Vec::new();
    for a in content.split(",") {
        let a = a.trim();
        if a == "" {
            continue;
        }
        let mut splitage = a.split("-");
        let first: u64 = splitage.next().unwrap().parse().unwrap();
        let second: u64 = splitage.next().unwrap().parse().unwrap();
        output.push((first, second));
    }
    Ok(output)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_invalid() {
        assert_eq!(is_invalid(99), true);
        assert_eq!(is_invalid(108), false);
        assert_eq!(is_invalid(6464), true);
        assert_eq!(is_invalid(10501), false);
        assert_eq!(is_invalid2(1010), true);
        assert_eq!(is_invalid2(222222), true);
        assert_eq!(is_invalid2(38593859), true);
        assert_eq!(is_invalid2(2121212121), true);
    }
}
