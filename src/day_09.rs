use crate::common::DaySpec;

pub const DAY_NINE: DaySpec<u64, u64> = DaySpec {
    day_num: 9,
    part_1_name: "filesystem checksum",
    part_1,
    part_2_name: "filesystem checksum without fragmentation",
    part_2,
};

fn part_1(input: &str) -> u64 {
    let mut disk_contents = parse_input(input);
    let mut start = 0;
    let mut end = disk_contents.len() - 1;
    while start < end {
        match (disk_contents[start], disk_contents[end]) {
            (_, None) => end -= 1,
            (Some(_), _) => start += 1,
            (None, Some(_)) => {
                disk_contents[start] = disk_contents[end];
                disk_contents[end] = None;
                end -= 1;
                start += 1;
            }
        }
    }
    generate_checksum(&disk_contents)
}

fn part_2(input: &str) -> u64 {
    let mut disk_contents = parse_input(input);
    let mut start = 0;
    let mut end = disk_contents.len() - 1;
    let mut file_id = None;
    let mut file_size = 0;
    while start < end {
        if let None = file_id {
            match (disk_contents[start], disk_contents[end]) {
                (_, None) => end -= 1,
                (Some(_), _) => start += 1,
                (None, Some(_)) => file_id = disk_contents[end],
            }
        } else {
            if file_id == disk_contents[end] {
                file_size += 1;
                end -= 1;
            } else {
                let gap_start = find_gap(file_size, start, end + 1, &disk_contents);
                if let Some(gap_start) = gap_start {
                    for i in 0..file_size {
                        disk_contents[gap_start + i] = disk_contents[end + 1 + i];
                        disk_contents[end + 1 + i] = None;
                    }
                }
                file_id = None;
                file_size = 0;
            }
        }
    }
    generate_checksum(&disk_contents)
}

fn generate_checksum(disk_contents: &[Option<u16>]) -> u64 {
    let mut checksum = 0;
    let mut count = 0;
    for &block in disk_contents {
        if let Some(block) = block {
            checksum += count * u64::try_from(block).unwrap();
        }
        count += 1;
    }
    checksum
}

fn find_gap(
    file_size: usize,
    mut start: usize,
    end: usize,
    disk_contents: &[Option<u16>]
) -> Option<usize> {
    let mut gap_size = 0;
    while start < end {
        if let Some(_) = disk_contents[start] {
            gap_size = 0;
        } else {
            gap_size += 1;
            if gap_size >= file_size {
                return Some(start - (gap_size - 1));
            }
        }
        start += 1;
    }
    None
}

fn parse_input(input: &str) -> Vec<Option<u16>> {
    let mut list = Vec::new();
    for (i, c) in input.chars().enumerate() {
        let size = c.to_digit(10).unwrap();
        let id = if i % 2 == 0 {
            Some(u16::try_from(i / 2).unwrap())
        } else {
            None
        };
        for _ in 0..size {
            list.push(id);
        }
    }
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn part_1_sample() {
        assert_eq!(1928, part_1(INPUT));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(2858, part_2(INPUT));
    }
}