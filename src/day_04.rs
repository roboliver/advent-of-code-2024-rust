use crate::common::DaySpec;

pub const DAY_FOUR: DaySpec<usize, usize> = DaySpec {
    day_num: 4,
    part_1_name: "XMAS matches",
    part_1,
    part_2_name: "X-MAS matches",
    part_2,
};

fn part_1(input: &str) -> usize {
    let word_search = parse_input(input);
    do_part(&word_search, &xmas_arrangements(), 'X')
}

fn part_2(input: &str) -> usize {
    let word_search = parse_input(input);
    do_part(&word_search, &crossmas_arrangements(), 'A')
}

fn do_part(word_search: &[Vec<char>], arrangements: &[Vec<LetterPosition>], starting_letter: char) -> usize {
    let mut matches = 0;
    for row in 0..word_search.len() {
        for col in 0..word_search[row].len() {
            let letter = word_search[row][col];
            if letter == starting_letter {
                matches += arrangements.iter()
                    .filter(|it| check_arrangement(word_search, it, row, col))
                    .count();
            }
        }
    }
    matches
}

fn check_arrangement(
    word_search: &[Vec<char>],
    arrangement: &[LetterPosition],
    start_row: usize,
    start_col: usize) -> bool {
    for letter_position in arrangement {
        let row = usize::try_from(
            isize::try_from(start_row).unwrap() + letter_position.row_adjust
        );
        let col = usize::try_from(
            isize::try_from(start_col).unwrap() + letter_position.col_adjust
        );
        let (Ok(row_u), Ok(col_u)) = (row, col) else {
            return false;
        };
        if row_u >= word_search.len() ||
            col_u >= word_search.len() ||
            word_search[row_u][col_u] != letter_position.letter {
            return false;
        }
    }
    true
}

struct LetterPosition {
    row_adjust: isize,
    col_adjust: isize,
    letter: char
}

struct XmasArrangement {
    m_row: isize,
    m_col: isize,
    a_row: isize,
    a_col: isize,
    s_row: isize,
    s_col: isize,
}

fn xmas_arrangements() -> Vec<Vec<LetterPosition>> {
    let arrangements = vec![
        XmasArrangement { m_row:  0, m_col:  1, a_row:  0, a_col:  2, s_row:  0, s_col:  3 }, // up
        XmasArrangement { m_row: -1, m_col:  0, a_row: -2, a_col:  0, s_row: -3, s_col:  0 }, // right
        XmasArrangement { m_row:  1, m_col:  0, a_row:  2, a_col:  0, s_row:  3, s_col:  0 }, // down
        XmasArrangement { m_row:  0, m_col: -1, a_row:  0, a_col: -2, s_row:  0, s_col: -3 }, // left
        XmasArrangement { m_row: -1, m_col:  1, a_row: -2, a_col:  2, s_row: -3, s_col:  3 }, // up-right
        XmasArrangement { m_row:  1, m_col:  1, a_row:  2, a_col:  2, s_row:  3, s_col:  3 }, // down-right
        XmasArrangement { m_row:  1, m_col: -1, a_row:  2, a_col: -2, s_row:  3, s_col: -3 }, // down-left
        XmasArrangement { m_row: -1, m_col: -1, a_row: -2, a_col: -2, s_row: -3, s_col: -3 }, // up-left
    ];

    arrangements.iter().map(|arr| {
        vec![
            LetterPosition { row_adjust: arr.m_row, col_adjust: arr.m_col, letter: 'M' },
            LetterPosition { row_adjust: arr.a_row, col_adjust: arr.a_col, letter: 'A' },
            LetterPosition { row_adjust: arr.s_row, col_adjust: arr.s_col, letter: 'S' },
        ]
    }).collect()
}

struct CrossMasArrangement {
    m1_row: isize,
    m1_col: isize,
    s1_row: isize,
    s1_col: isize,
    m2_row: isize,
    m2_col: isize,
    s2_row: isize,
    s2_col: isize,
}

fn crossmas_arrangements() -> Vec<Vec<LetterPosition>> {
    let arrangements = vec![
        CrossMasArrangement {
            m1_row: -1, m1_col: -1, s1_row: -1, s1_col:  1, // left-to-right
            m2_row:  1, m2_col: -1, s2_row:  1, s2_col:  1,
        },
        CrossMasArrangement {
            m1_row: -1, m1_col: -1, s1_row:  1, s1_col: -1, // top-to-bottom
            m2_row: -1, m2_col:  1, s2_row:  1, s2_col:  1,
        },
        CrossMasArrangement {
            m1_row: -1, m1_col:  1, s1_row: -1, s1_col: -1, // right-to-left
            m2_row:  1, m2_col:  1, s2_row:  1, s2_col: -1,
        },
        CrossMasArrangement {
            m1_row:  1, m1_col: -1, s1_row: -1, s1_col: -1, // bottom-to-top
            m2_row:  1, m2_col:  1, s2_row: -1, s2_col:  1,
        },
    ];

    arrangements.iter().map(|arr| {
        vec![
            LetterPosition { row_adjust: arr.m1_row, col_adjust: arr.m1_col, letter: 'M' },
            LetterPosition { row_adjust: arr.s1_row, col_adjust: arr.s1_col, letter: 'S' },
            LetterPosition { row_adjust: arr.m2_row, col_adjust: arr.m2_col, letter: 'M' },
            LetterPosition { row_adjust: arr.s2_row, col_adjust: arr.s2_col, letter: 'S' },
        ]
    }).collect()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part_1_sample() {
        assert_eq!(18, part_1(INPUT));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(9, part_2(INPUT));
    }
}
