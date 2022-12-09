use std::ops::RangeInclusive;

/// # Panics
/// if input contains non parsable integers between '-' and ','
#[must_use]
pub fn get_ranges(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let output = input
        // group pairs
        .lines()
        .skip_while(|&line| line.is_empty())
        .map(str::trim)
        .flat_map(|group_pair| {
            group_pair
                .split(',')
                .flat_map(|group| group.split('-'))
                .skip_while(|&cleaning_section| cleaning_section.is_empty())
                .map(|cleaning_section| match cleaning_section.parse::<u32>() {
                    Ok(section_number) => section_number,
                    Err(why) => panic!("{}", why),
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<u32>>();

    let mut ranges = vec![];
    for el in output.chunks(4) {
        ranges.push((el[0]..=el[1], el[2]..=el[3]));
    }
    ranges
}

#[must_use]
pub fn ranges_full_overlap(ranges: &(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    fn smaller(ranges: &(RangeInclusive<u32>, RangeInclusive<u32>)) -> &RangeInclusive<u32> {
        let diff0 = ranges.0.end() - ranges.0.start();
        let diff1 = ranges.1.end() - ranges.1.start();

        if diff0 < diff1 {
            &ranges.0
        } else {
            &ranges.1
        }
    }
    fn larger(ranges: &(RangeInclusive<u32>, RangeInclusive<u32>)) -> &RangeInclusive<u32> {
        let diff0 = ranges.0.end() - ranges.0.start();
        let diff1 = ranges.1.end() - ranges.1.start();

        if diff0 >= diff1 {
            &ranges.0
        } else {
            &ranges.1
        }
    }

    let inner = smaller(ranges);
    let container = larger(ranges);

    return container.start() <= inner.start() && container.end() >= inner.end();
}

#[must_use]
pub fn ranges_partial_overlap(ranges: &(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    if ranges.1.contains(ranges.0.start()) || ranges.1.contains(ranges.0.end()) {
        // range 0's edges is inside range 1
        return true;
    }
    if ranges.0.contains(ranges.1.start()) || ranges.0.contains(ranges.1.end()) {
        // range 1's edges is inside range 0
        return true;
    }
    false
}

#[must_use]
pub fn count_num_ranges_with_full_overlap(
    ranges: &[(RangeInclusive<u32>, RangeInclusive<u32>)],
) -> u32 {
    let output = ranges.iter().fold(0, |acc, ranges| {
        if ranges_full_overlap(ranges) {
            acc + 1
        } else {
            acc
        }
    });
    output
}
#[must_use]
pub fn count_num_ranges_with_partial_overlap(
    ranges: &[(RangeInclusive<u32>, RangeInclusive<u32>)],
) -> u32 {
    let output = ranges.iter().fold(0, |acc, ranges| {
        if ranges_partial_overlap(ranges) {
            acc + 1
        } else {
            acc
        }
    });
    output
}

#[cfg(test)]
mod tests {
    use crate::day_4::{
        count_num_ranges_with_full_overlap, count_num_ranges_with_partial_overlap, get_ranges,
    };

    const INPUT: &str = include_str!("../puzzle_input/day_4.txt");
    const EXAMPLE_INPUT: &str = r"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    const ANSWER: [u32; 2] = [526, 886];
    const EXAMPLE_ANSWER: [u32; 2] = [2, 4];

    #[test]
    fn example_1() {
        let ranges = get_ranges(EXAMPLE_INPUT);
        let output = count_num_ranges_with_full_overlap(&ranges);
        assert_eq!(output, EXAMPLE_ANSWER[0]);
    }

    #[test]
    fn problem_1() {
        let ranges = get_ranges(INPUT);
        let output = count_num_ranges_with_full_overlap(&ranges);
        assert_eq!(output, ANSWER[0]);
    }

    #[test]
    fn example_2() {
        let ranges = get_ranges(EXAMPLE_INPUT);
        let output = count_num_ranges_with_partial_overlap(&ranges);
        assert_eq!(output, EXAMPLE_ANSWER[1]);
    }

    #[test]
    fn problem_2() {
        let ranges = get_ranges(INPUT);
        let output = count_num_ranges_with_partial_overlap(&ranges);
        assert_eq!(output, ANSWER[1]);
    }
}
