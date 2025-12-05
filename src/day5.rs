use std::{fs, io, usize};
#[allow(unused)]
pub fn solve_input() -> io::Result<()> {
    let (ranges, ids) = parse_input("input.txt")?;
    let ranges = fix_range(ranges);
    let solution = 0; // this is for I don't send uncompilable code to github
    // let solution = solve1(ranges, ids); // uncomment this for solution 1
    // let solution = solve2(ranges); //uncomment this for solution 2
    println!("Solution is {solution}");
    Ok(())
}

#[allow(unused)]
fn solve2(ranges: Vec<(usize, usize)>) -> usize {
    let mut total = 0;
    for (a, b) in ranges {
        total += b - a + 1;
    }
    return total;
}

#[allow(unused)]
pub fn solve1(ranges: Vec<(usize, usize)>, ids: Vec<usize>) -> usize {
    let mut total = 0;
    for id in ids.iter() {
        let res = ranges.binary_search_by(|(a, b)| match id {
            id if a <= id && id <= b => std::cmp::Ordering::Equal,
            id if id < a => std::cmp::Ordering::Greater,
            id if id > b => std::cmp::Ordering::Less,
            _ => panic!("This should never happpen range:{a}-{b} id:{id}"),
        });
        total += match res {
            Ok(_) => 1,
            Err(_) => 0,
        };
    }

    return total;
}
pub fn fix_range(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    ranges.sort_by(|(a, _), (b, _)| a.cmp(b));
    let mut non_overlap: Vec<(usize, usize)> = Vec::with_capacity(ranges.len());
    let mut i = 0;
    while i < ranges.len() {
        let (min_start, mut max_end) = ranges[i];
        let mut j = i + 1;
        while j < ranges.len() {
            let (start, end) = ranges[j];
            if max_end < start {
                break;
            }
            max_end = end.max(max_end);
            j += 1;
        }
        non_overlap.push((min_start, max_end));
        i = j;
    }
    non_overlap
}
pub fn parse_input(file: &str) -> io::Result<(Vec<(usize, usize)>, Vec<usize>)> {
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut ids: Vec<usize> = Vec::new();
    let mut seperated: bool = false;
    let content = fs::read_to_string(file)?;
    for line in content.lines() {
        if line.trim() == "" {
            seperated = true;
            continue;
        }
        if !seperated {
            let (a, b) = line.split_once('-').unwrap();
            if b < a {
                panic!("How is that possible: {a}-{b}");
            }
            ranges.push((a.trim().parse().unwrap(), b.trim().parse().unwrap()));
            continue;
        }
        ids.push(line.trim().parse().unwrap());
    }
    Ok((ranges, ids))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_overlap() {
        let range = vec![(2, 5), (1, 2), (1, 4), (5, 6), (4, 7), (9, 10)];
        assert_eq!(fix_range(range), vec![(1, 7), (9, 10)]);
    }
}
