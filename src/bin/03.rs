#![feature(iter_array_chunks)]
use std::collections::{HashMap, HashSet};

fn get_priorities() -> HashMap<char, u32> {
    let priorities = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, val)| (val, (i as u32) + 1));

    HashMap::from_iter(priorities)
}

fn find_intersects(line: &str) -> Vec<char> {
    let (bag1, bag2) = line.split_at(line.len() / 2);

    let bag1 = HashSet::<char>::from_iter(bag1.chars());
    let bag2 = HashSet::<char>::from_iter(bag2.chars());

    bag1.intersection(&bag2).copied().collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let priorities = get_priorities();

    let total = lines.flat_map(find_intersects)
        .map(|item| priorities.get(&item).expect("Unknown bag item!"))
        .sum();

    Some(total)
}

fn find_group_intersect(group: [&str; 3]) -> char {
    let elf1 = HashSet::<char>::from_iter(group[0].chars());
    let elf2 = HashSet::<char>::from_iter(group[1].chars());
    let elf3 = HashSet::<char>::from_iter(group[2].chars());

    let first_two = HashSet::<char>::from_iter(elf1.intersection(&elf2).copied());
    let mut badge = first_two.intersection(&elf3).copied();

    badge.next().expect("Somehow no badge!")
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let priorities = get_priorities();

    let elf_groups = lines.array_chunks::<3>();

    let total = elf_groups.map(find_group_intersect)
        .map(|badge| priorities.get(&badge).expect("Unknown bag item!"))
        .sum();

    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
