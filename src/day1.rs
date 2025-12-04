use std::{fs, io};
fn negative_modulo(instruction: i16, divident: i16) -> i16 {
    if instruction >= 0 {
        return instruction % divident;
    }
    let result: f32 = (instruction.abs() as f32) / (divident as f32);
    return instruction + (result.ceil() as i16) * divident;
}
pub fn solve_input() -> io::Result<()> {
    let content = fs::read_to_string("input_test.txt")?;
    let mut points: i16 = 50;
    let mut counter_zeroes = 0;
    for line in content.lines() {
        let instruction = &line[0..1];
        let turn: i16 = (&line[1..]).parse().unwrap();
        let mut total_zeroes = 0;
        print!("start is {points}");
        if instruction == "R" {
            total_zeroes += (((points + turn) as f32) / 100.0).floor() as u32;
            points = (points + turn) % 100;
        }
        if instruction == "L" {
            match points {
                x if x > turn => total_zeroes = 0,
                x if x == turn => total_zeroes = 1,
                0 => total_zeroes = (((points - turn) as f32) / 100.0).abs().floor() as u32, // if we start at 0 we don't need to add that extra one for passing 0
                _ => total_zeroes = (((points - turn) as f32) / 100.0).abs().floor() as u32 + 1,
            }
            points = negative_modulo(points - turn, 100);
        }
        println!(" after is {points}, from instruction {line} total_zeroes is: {total_zeroes}");
        counter_zeroes += total_zeroes
    }
    println!("Counter is {}", counter_zeroes);
    Ok(())
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_negative_modulo() {
        assert_eq!(negative_modulo(-18, 99), 81);
        assert_eq!(negative_modulo(-213, 99), 84);
        assert_eq!(negative_modulo(213, 99), 15);
    }
}
