use nom::{
    bytes::complete::take,
    character::{complete::digit0, streaming::alpha1},
    sequence::tuple,
    IResult,
};
use rust_aoc2023::get_puzzle_input_string;
use std::collections::HashMap;

type LensBoxes<'a> = HashMap<u8, Vec<(&'a str, u8)>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operation<'a> {
    FocalLength(u8),
    Remove(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
struct BoxInst<'a> {
    label: &'a str,
    box_id: u8,
    op: Operation<'a>,
}

/// Example:
/// ```
/// "qp=3" -> BoxInst { label: "qp", box_id: 1, op: Operation::FocalLength(3) }
/// ```
fn parse_box(input: &str) -> IResult<&str, BoxInst> {
    let (_, (label, op, focal_length)) = tuple((alpha1, take(1usize), digit0))(input)?;
    let box_id = hash(label).try_into().unwrap();
    let op = match op {
        "=" => Operation::FocalLength(focal_length.parse().unwrap()),
        "-" => Operation::Remove(label),
        _ => panic!("unrecognized operation char {op}"),
    };
    Ok(("", BoxInst { label, box_id, op }))
}

impl<'a> From<&'a str> for BoxInst<'a> {
    fn from(value: &'a str) -> Self {
        let (_, boxinst) = parse_box(value).expect("error with parse");
        boxinst
    }
}

fn hash(string: &str) -> usize {
    let mut total = 0;
    for c in string.chars() {
        total = ((total + c as usize) * 17) % 256;
    }
    total
}

fn part_1(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

/// Here we use the `position` method of `Vec` to figure out whether & where
/// to insert (or replace) lenses into a box. Removal uses the `remove` method
/// which is similar.
fn part_2<'a>(input: &'a str, lens_boxes: &mut LensBoxes<'a>) -> usize {
    let box_instructions: Vec<BoxInst> = input.split(',').map(|s| s.into()).collect();
    for inst in box_instructions {
        // println!("processing: {inst:?}");
        let BoxInst { label, box_id, op } = inst;
        match op {
            // Check if this box has had anything in it yet
            Operation::FocalLength(focal_len) => match lens_boxes.get_mut(&box_id) {
                // if it has, search the box for the label & focal_len
                Some(entry) => {
                    match entry
                        .iter()
                        .map(|(label, _)| label)
                        .position(|l| l == &label)
                    {
                        // Replace it if it exists
                        Some(idx) => entry[idx] = (label, focal_len),
                        // Add it to the end of the exisiting `Vec`
                        None => entry.push((label, focal_len)),
                    }
                }
                // If there's nothing in this box yet, create it with the (label, focal_len)
                None => {
                    lens_boxes.insert(box_id, vec![(label, focal_len)]);
                }
            },
            Operation::Remove(label) => {
                // Same process as above, find the label and if it exists remove it
                // from the `Vec`
                if let Some(lenses) = lens_boxes.get_mut(&box_id) {
                    if let Some(idx) = lenses
                        .iter()
                        .map(|(label, _)| label)
                        .position(|l| l == &label)
                    {
                        _ = lenses.remove(idx);
                    }
                }
            }
        }
    }
    let mut total = 0usize;
    for (box_id, lens_box) in lens_boxes.iter() {
        total += lens_box
            .iter()
            .enumerate()
            .map(|(i, (_l, focal_len))| (*box_id as usize + 1) * (i + 1) * *focal_len as usize)
            .sum::<usize>()
    }
    total
}

fn main() {
    let input = get_puzzle_input_string(15).expect("I/O Error");
    // have to remove newline characters
    let input = input.strip_suffix('\n').unwrap();
    println!("Part 1: {}", part_1(input));
    let mut lens_boxes: LensBoxes = HashMap::new();
    println!("Part 2: {}", part_2(input, &mut lens_boxes));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_part_1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part_1(input), 1320);
    }

    #[test]
    fn test_parse_box() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let boxes: Vec<BoxInst> = input.split(',').map(|s| s.into()).collect();
        assert_eq!(boxes.len(), 11);
        assert_eq!(
            boxes[0],
            BoxInst {
                label: "rn",
                box_id: 0,
                op: Operation::FocalLength(1)
            }
        );
        assert_eq!(
            boxes[4],
            BoxInst {
                label: "qp",
                box_id: 1,
                op: Operation::Remove("qp")
            }
        );
    }

    #[test]
    fn test_part_2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let mut lens_boxes: LensBoxes = HashMap::new();
        let val = part_2(input, &mut lens_boxes);
        assert_eq!(val, 145);
    }
}
