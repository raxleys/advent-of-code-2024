use crate::utils;
use std::collections::HashSet;
use utils::{Res, Soln};

#[allow(dead_code)]
fn print_matrix(matrix: &Vec<Vec<char>>) {
    matrix
        .iter()
        .map(|line| line.iter().collect())
        .for_each(|line: String| println!("{line}"));
}

fn count_horiz_fwd(line: &Vec<char>, word: &str) -> usize {
    let chars: Vec<_> = word.chars().collect();
    let mut count = 0;
    let mut it = 0;
    while it < line.len() {
        let skip = line[it..].iter().take_while(|&&c| c != chars[0]).count();

        if it + skip >= line.len() {
            break;
        }

        it += skip;

        let next = line[it..].iter().take(word.len()).collect::<String>();

        if next == word {
            count += 1;
            it += word.len();
        } else {
            it += 1;
        }
    }
    count
}

fn rotate_cw(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut flipped: Vec<Vec<char>> = (0..matrix[0].len())
        .map(|_| vec!['.'; matrix.len()])
        .collect();
    let len = flipped[0].len();
    for (i, line) in matrix.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            flipped[j][len - i - 1] = *c;
        }
    }
    flipped
}

fn part1(input: &Vec<String>) -> Res<i32> {
    let mut matrix: Vec<_> = input
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    let mut count = 0;
    for _ in 0..=3 {
        // Count horizontal instances
        matrix.iter().for_each(|line| {
            count += count_horiz_fwd(&line, "XMAS");
        });

        // Count diagonal instances
        for word in ["XMAS"] {
            let first_char = word.chars().next().unwrap();
            let rest = word.chars().skip(1).collect::<String>();
            for (i, line) in matrix.iter().enumerate() {
                for (j, c) in line.iter().enumerate() {
                    if *c != first_char {
                        continue;
                    }

                    if j + rest.len() >= line.len() || i + rest.len() >= matrix.len() {
                        continue;
                    }

                    let mut char_it = rest.chars();
                    let sec = char_it.next().unwrap();
                    let third = char_it.next().unwrap();
                    let fourth = char_it.next().unwrap();
                    if matrix[i + 1][j + 1] == sec
                        && matrix[i + 2][j + 2] == third
                        && matrix[i + 3][j + 3] == fourth
                    {
                        count += 1;
                    }
                }
            }
        }

        matrix = rotate_cw(&matrix);
    }

    Ok(count.try_into().unwrap())
}

fn part2(input: &Vec<String>) -> Res<i32> {
    let matrix: Vec<_> = input
        .into_iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut count = 0;
    let first_lets: HashSet<char> = vec!['M', 'S'].into_iter().collect();
    for (i, line) in matrix.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            if !first_lets.contains(&matrix[i][j]) {
                continue;
            }

            if i + 2 >= matrix.len() || j + 2 >= matrix[0].len() {
                continue;
            }

            if matrix[i + 1][j + 1] != 'A'
                || !first_lets.contains(&matrix[i][j + 2])
                || !first_lets.contains(&matrix[i + 2][j])
                || !first_lets.contains(&matrix[i + 2][j + 2])
            {
                continue;
            }

            let mut n_ms = 0;
            for k in [0, 2].iter() {
                for l in [0, 2].iter() {
                    if matrix[i + k][j + l] == 'M' {
                        n_ms += 1;
                    }
                }
            }

            if n_ms == 2 && matrix[i][j] != matrix[i + 2][j + 2] {
                count += 1;
            }
        }
    }

    Ok(count)
}

pub const DAY4_SOLN: Soln = Soln {
    first: part1,
    second: part2,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fwd() {
        let input = vec![
            "....XMAS....".to_string(),
            "............".to_string(),
            "............".to_string(),
            "............".to_string(),
            "............".to_string(),
            "............".to_string(),
        ];

        assert_eq!(part1(&input).unwrap(), 1);
    }

    #[test]
    fn test_rev() {
        let input = vec![
            "....SAMX....".to_string(),
            "............".to_string(),
            "............".to_string(),
            "............".to_string(),
            "............".to_string(),
            "............".to_string(),
        ];

        assert_eq!(part1(&input).unwrap(), 1);
    }

    #[test]
    fn test_down() {
        let input = vec![
            "...........X".to_string(),
            "...........M".to_string(),
            "...........A".to_string(),
            "...........S".to_string(),
            "............".to_string(),
            "............".to_string(),
        ];

        assert_eq!(part1(&input).unwrap(), 1);
    }

    #[test]
    fn test_up() {
        let input = vec![
            "............".to_string(),
            "S...........".to_string(),
            "A...........".to_string(),
            "M...........".to_string(),
            "X...........".to_string(),
            "............".to_string(),
        ];

        assert_eq!(part1(&input).unwrap(), 1);
    }

    #[test]
    fn test_diag_lr() {
        let input = vec![
            "............".to_string(),
            "............".to_string(),
            "...X........".to_string(),
            "....M.......".to_string(),
            ".....A......".to_string(),
            "......S.....".to_string(),
        ];

        assert_eq!(part1(&input).unwrap(), 1);
    }

    #[test]
    fn test_diag_ur() {
        let input = vec![
            "............".to_string(),
            "......S.....".to_string(),
            ".....A......".to_string(),
            "....M.......".to_string(),
            "...X........".to_string(),
            "............".to_string(),
        ];

        assert_eq!(part1(&input).unwrap(), 1);
    }

    #[test]
    fn test_diag_ll() {
        let input = vec![
            "............".to_string(),
            "......X.....".to_string(),
            ".....M......".to_string(),
            "....A.......".to_string(),
            "...S........".to_string(),
            "............".to_string(),
        ];

        assert_eq!(part1(&input).unwrap(), 1);
    }

    #[test]
    fn test_diag_ul() {
        let input = vec![
            "............".to_string(),
            "...S........".to_string(),
            "....A.......".to_string(),
            ".....M......".to_string(),
            "......X.....".to_string(),
            "............".to_string(),
        ];

        assert_eq!(part1(&input).unwrap(), 1);
    }

    #[test]
    fn test_dummy_simple() {
        let input = vec![
            ".....XMAS.".to_string(),
            ".SAMX.....".to_string(),
            "..........".to_string(),
            ".........X".to_string(),
            "XMASAMX..M".to_string(),
            ".........A".to_string(),
            ".........S".to_string(),
            ".........A".to_string(),
            ".........M".to_string(),
            ".....XMASX".to_string(),
        ];

        assert_eq!(part1(&input).unwrap(), 7);
    }

    #[test]
    fn test_xx() {
        let input = vec![
            "....XXMAS.".to_string(),
            "..........".to_string(),
            "..........".to_string(),
            "..........".to_string(),
        ];

        assert_eq!(part1(&input).unwrap(), 1);
    }

    #[test]
    fn test_dummy() {
        let input = vec![
            "....XXMAS.".to_string(),
            ".SAMXMS...".to_string(),
            "...S..A...".to_string(),
            "..A.A.MS.X".to_string(),
            "XMASAMX.MM".to_string(),
            "X.....XA.A".to_string(),
            "S.S.S.S.SS".to_string(),
            ".A.A.A.A.A".to_string(),
            "..M.M.M.MM".to_string(),
            ".X.X.XMASX".to_string(),
        ];

        assert_eq!(part1(&input).unwrap(), 18);
    }

    #[test]
    fn test_2_simple() {
        let input = vec!["M.S".to_string(), ".A.".to_string(), "M.S".to_string()];

        assert_eq!(part2(&input).unwrap(), 1);

        let input = vec!["S.M".to_string(), ".A.".to_string(), "M.S".to_string()];

        assert_eq!(part2(&input).unwrap(), 0);

        let input = vec!["S.S".to_string(), ".A.".to_string(), "M.M".to_string()];

        assert_eq!(part2(&input).unwrap(), 1);

        let input = vec!["S.M".to_string(), ".A.".to_string(), "S.M".to_string()];

        assert_eq!(part2(&input).unwrap(), 1);

        let input = vec!["M.M".to_string(), ".A.".to_string(), "S.M".to_string()];

        assert_eq!(part2(&input).unwrap(), 0);

        let input = vec!["S.S".to_string(), ".A.".to_string(), "S.M".to_string()];

        assert_eq!(part2(&input).unwrap(), 0);

        let input = vec!["M.M".to_string(), ".A.".to_string(), "M.S.".to_string()];

        assert_eq!(part2(&input).unwrap(), 0);

        let input = vec!["M.M".to_string(), ".A.".to_string(), "X.S.".to_string()];

        assert_eq!(part2(&input).unwrap(), 0);

        let input = vec!["S.M".to_string(), ".A.".to_string(), "M.S.".to_string()];

        assert_eq!(part2(&input).unwrap(), 0);
    }

    #[test]
    fn test_2_dummy() {
        let input = vec![
            ".M.S......".to_string(),
            "..A..MSMS.".to_string(),
            ".M.S.MAA..".to_string(),
            "..A.ASMSM.".to_string(),
            ".M.S.M....".to_string(),
            "..........".to_string(),
            "S.S.S.S.S.".to_string(),
            ".A.A.A.A..".to_string(),
            "M.M.M.M.M.".to_string(),
            "..........".to_string(),
        ];

        assert_eq!(part2(&input).unwrap(), 9);

        let input = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];

        assert_eq!(part2(&input).unwrap(), 9);
    }
}
