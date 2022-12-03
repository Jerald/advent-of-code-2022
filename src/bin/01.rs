use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let elves_items = input.lines().group_by(|line| !line.is_empty());

    let elves_totals = elves_items
        .into_iter()
        .filter(|&(is_elf, _)| is_elf)
        .map(|(_, group)| {
            group
                .map(|count| count.parse::<u32>().unwrap())
                .sum::<u32>()
        });

    elves_totals.max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves_items = input.lines().group_by(|line| !line.is_empty());

    let elves_totals = elves_items
        .into_iter()
        .filter(|&(is_elf, _)| is_elf)
        .map(|(_, group)| {
            group
                .map(|count| count.parse::<u32>().unwrap())
                .sum::<u32>()
        });

    Some(elves_totals.sorted().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
