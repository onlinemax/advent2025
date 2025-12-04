use std::{fs, io};
pub fn solve_input() -> io::Result<()> {
    let content = fs::read_to_string("input.txt")?;
    let (mut grid, rows, cols) = parse_input(content);

    let check_bound =
        |row: i32, col: i32| 0 <= row && row < (rows as i32) && 0 <= col && col < (cols as i32);

    let get_col = |pos: u32| ((pos / cols as u32) as i32, (pos % cols as u32) as i32);

    let directions = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    let mut answer = 0;
    let mut removable_papers: Vec<usize> = Vec::new();
    loop {
        let removable_papers = &mut removable_papers;
        for i in 0..grid.len() {
            if grid[i] != b'@' {
                continue;
            }
            let (row, col) = get_col(i as u32);
            let mut count = 0;
            for (dr, dc) in directions {
                let nrow = row + dr;
                let ncol = col + dc;
                let npos = nrow * cols as i32 + ncol;

                if !check_bound(nrow, ncol) || grid[npos as usize] != b'@' {
                    continue;
                }
                count += 1;
            }
            if count < 4 {
                removable_papers.push(i);
            }
        }
        answer += removable_papers.len();
        // for part 1 just remove this comment
        // break;
        for pos in removable_papers.iter() {
            grid[*pos] = b'.';
        }
        if removable_papers.len() == 0 {
            break;
        }
        removable_papers.clear();
    }
    println!("There are {answer} possible papers rolls");

    Ok(())
}
fn parse_input(input: String) -> (Vec<u8>, usize, usize) {
    let input = input.as_bytes();
    let cols = input.iter().position(|a| *a == b'\n').unwrap();
    let input: Vec<u8> = input.split(|a| *a == b'\n').flatten().cloned().collect();
    let rows = input.len() / cols;

    return (input, rows, cols);
}
