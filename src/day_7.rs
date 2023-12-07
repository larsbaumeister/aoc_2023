use std::{error::Error, io::{BufReader, BufRead}};

pub fn solve() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(std::fs::File::open("input/day_7.txt")?);

    let mut hands: Vec<(String, u32, u32)> = vec![];
    for line in reader.lines() {
        let (hand, score) = parse_line(&line?);
        let value = calc_hand_value(&hand);
        hands.push((hand, score, value));
    }

    // sort the hands by their value
    hands.sort_unstable_by_key(|h| h.2);
    for (idx, hand) in hands.iter().enumerate() {
        println!("{:#04}, {} {:#04} {:#08x}", idx + 1, hand.0, hand.1, hand.2);
    }
    let mut total_score: u64 = 0;
    for i in 0..hands.len() {
        let (_, score, _) = &hands[i];
        let x = score * (i as u32 + 1);
        total_score += x as u64;
    }
    println!("Total score: {}", total_score);
    Ok(())
}

fn calc_hand_value(hand: &str) -> u32 {
    // group the characters by their count
    let mut counts: Vec<(char, u32)> = vec![];
    for c in hand.chars() {
        let mut found = false;
        for i in 0..counts.len() {
            if counts[i].0 == c {
                counts[i].1 += 1;
                found = true;
                break;
            }
        }
        if !found {
            counts.push((c, 1));
        }
    }


    // sort the counts by their count, then by their value
    counts.sort_unstable_by_key(|c| (u32::MAX - c.1, u32::MAX - get_card_value(c.0)));

    // now calculate the value of the hand. the most important part is the type of hand, which is
    // determined by the cards in the hand. eg. a full house is 0x700000, a flush is 0x600000, etc.
    // the next important part is the value of the first card in the hand, the next important part
    // is the value of the second card in the hand, etc.
    let mut type_value = 0x300000;
    if counts[0].1 == 5 {
        type_value = 0x900000;
    } else if counts[0].1 == 4 {
        type_value = 0x800000;
    } else if counts[0].1 == 3 && counts[1].1 == 2 {
        type_value = 0x700000;
    } else if counts[0].1 == 3 {
        type_value = 0x600000;
    } else if counts[0].1 == 2 && counts[1].1 == 2 {
        type_value = 0x500000;
    } else if counts[0].1 == 2 {
        type_value = 0x400000;
    }

    type_value  + get_card_value(hand.chars().nth(0).unwrap()) * 0x10000 
                + get_card_value(hand.chars().nth(1).unwrap()) * 0x1000 
                + get_card_value(hand.chars().nth(2).unwrap()) * 0x100
                + get_card_value(hand.chars().nth(3).unwrap()) * 0x10 
                + get_card_value(hand.chars().nth(4).unwrap())
}


/// Returns the value of a card. The value is a number from 0x0 to 0xD, where 0x0 is a 1, 0x1 is a
/// 2, 0x2 is a 3, etc.
/// @param c The card to get the value of.
/// @return The value of the card as u32 to prevent type casting later.
fn get_card_value(c: char) -> u32 {
    match c {
        '1' => 0x0,
        '2' => 0x1,
        '3' => 0x2,
        '4' => 0x3,
        '5' => 0x4,
        '6' => 0x5,
        '7' => 0x6,
        '8' => 0x7,
        '9' => 0x8,
        'T' => 0x9,
        'J' => 0xA,
        'Q' => 0xB,
        'K' => 0xC,
        'A' => 0xD,
        _ => panic!("Invalid card value: {}", c)
    }
}

fn parse_line(line: &str) -> (String, u32) {
    let split = line.split_ascii_whitespace().collect::<Vec<&str>>();
    return (split[0].to_owned(), split[1].parse::<u32>().unwrap())
}