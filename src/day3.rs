use std::{fs, io};
#[allow(unused)]
fn biggest_battery(line: &[u8]) -> usize {
    let mut a = 0;
    let mut b = 1;
    let n = line.len();
    for i in 0..n {
        let num = line[i] - b'0';
        let a_num = line[a] - b'0';
        let b_num = line[b] - b'0';
        if i != n - 1 && a_num < num {
            a = i;
            b = i + 1;
        }
        if b_num < num && a != i {
            b = i;
        }
    }
    let a_num: usize = (line[a] - b'0') as usize;
    let b_num: usize = (line[b] - b'0') as usize;
    return a_num * 10 + b_num;
}
#[allow(unused)]
fn biggest_battery2(line: &[u8]) -> u64 {
    let mut current: u64 = (line[0] - b'0').into();
    let n = line.len();
    for i in 1..n {
        let m = match current {
            0 => 0,
            c => c.ilog10() + 1,
        };
        let rest = n - i;
        let num: u64 = (line[i] - b'0').into();
        if (current % 10) < num {
            let max_delete = match rest {
                r if r > 12 => -1,
                r => r as isize + m as isize - 12,
            };
            let mut delete_count = 0;
            while (current % 10) < num
                && current > 0
                && (max_delete < 0 || delete_count < max_delete)
            {
                current /= 10;
                delete_count += 1;
            }
            current = current * 10 + num;
            continue;
        }
        if m < 12 {
            current = current * 10 + num;
        }
    }
    return current;
}
pub fn solve_input() -> io::Result<()> {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut sum: u64 = 0;
    let lines: Vec<&str> = content.lines().collect();
    for i in 0..lines.len() {
        let line = lines[i];
        let line = line.as_bytes();
        let biggest = biggest_battery2(line); // change that to biggest_battery2 for part 2
        sum += biggest as u64;
    }
    println!("The sum of biggest battery is {sum}");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biggest() {
        assert_eq!(biggest_battery("818181911112111".as_bytes()), 92);
        assert_eq!(biggest_battery("811111111111119".as_bytes()), 89);
        assert_eq!(biggest_battery("987654321111111".as_bytes()), 98);
        assert_eq!(biggest_battery2("987654321111111".as_bytes()), 987654321111);
        assert_eq!(biggest_battery2("811111111111119".as_bytes()), 811111111119);
        assert_eq!(biggest_battery2("234234234234278".as_bytes()), 434234234278);
        assert_eq!(biggest_battery2("818181911112111".as_bytes()), 888911112111);
    }
}
