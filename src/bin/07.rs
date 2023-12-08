use std::cmp::Ordering;
advent_of_code::solution!(7);

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
enum HandType {
    High,
    OnePair,
    TwoPairs,
    Three,
    Full,
    Four,
    Five,
}

fn get_card_type(card: [u64; 5]) -> HandType {
    // Get easy cases out of the way
    if card.windows(2).all(|a| a[0] == a[1]) {
        return HandType::Five;
    } else {
        // Create card histogram
        let mut card_count = vec![0u64; 15];
        for c in card {
            card_count[c as usize] += 1;
        }

        if card_count.contains(&4) {
            return HandType::Four;
        } else if card_count.contains(&3) && card_count.contains(&2) {
            return HandType::Full;
        } else if card_count.contains(&3) {
            return HandType::Three;
        } else if card_count.iter().filter(|&&n| n == 2).count() == 2 {
            return HandType::TwoPairs;
        } else if card_count.iter().filter(|&&n| n == 2).count() == 1 {
            return HandType::OnePair;
        } else {
            return HandType::High;
        }
    }
}

fn power_to_card(pow: u64) -> char {
    assert!(pow >= 2);
    assert!(pow <= 14);
    match pow {
        2..=9 => char::from_digit(pow as u32, 10).unwrap(),
        10 => 'T',
        11 => 'J',
        12 => 'Q',
        13 => 'K',
        14 => 'A',
        _ => {
            assert!(false);
            ' '
        }
    }
}

fn card_to_power(card: char) -> u64 {
    match card {
        c if c.is_digit(10) => c.to_digit(10).unwrap() as u64,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => {
            assert!(false);
            0
        }
    }
}

#[derive(std::cmp::PartialEq, std::cmp::Eq)]
struct Hand {
    cards: [u64; 5],
    bid: u64,
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let type_self = get_card_type(self.cards);
        let type_other = get_card_type(other.cards);
        if type_self < type_other {
            return Some(Ordering::Less);
        } else if type_self > type_other {
            return Some(Ordering::Greater);
        } else {
            // Compare cards one by one
            for (&c_s, &c_o) in self.cards.iter().zip(other.cards.iter()) {
                if c_s < c_o {
                    return Some(Ordering::Less);
                } else if c_s > c_o {
                    return Some(Ordering::Greater);
                }
            }
            // All equal
            assert!(false);
            return Some(Ordering::Equal);
        }
    }
}

impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[allow(dead_code)]
fn print_hand(hand: &Hand) {
    let cards: String = hand.cards.map(|p| power_to_card(p)).iter().collect();
    println!("cards: [{}], bid: {}", cards, hand.bid);
}

fn str_to_hand(hand_str: &str) -> Hand {
    let mut new_hand = Hand {
        cards: [0, 0, 0, 0, 0],
        bid: 0,
    };
    let mut it = hand_str.split_ascii_whitespace();
    let cards_chars: Vec<u64> = it
        .next()
        .unwrap()
        .chars()
        .map(|c| card_to_power(c))
        .collect();
    new_hand.cards = cards_chars.try_into().unwrap();
    new_hand.bid = u64::from_str_radix(it.next().unwrap(), 10).unwrap();

    new_hand
}

fn read_cards(input: &str) -> Vec<Hand> {
    input.lines().map(|l| str_to_hand(l)).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut hands = read_cards(input);
    hands.sort();

    let winnings: u64 = hands.iter().enumerate().map(|(r, hand)| {
        // print_hand(hand);
        // println!("rank {}", r + 1);
        ((r as u64) + 1) * hand.bid
    }).sum();

    Some(winnings)
}

fn power_to_card_joker(pow: u64) -> char {
    assert!(pow >= 1);
    assert!(pow <= 14);
    match pow {
        1 => 'J',
        2..=9 => char::from_digit(pow as u32, 10).unwrap(),
        10 => 'T',
        12 => 'Q',
        13 => 'K',
        14 => 'A',
        _ => {
            assert!(false);
            ' '
        }
    }
}

fn card_to_power_joker(card: char) -> u64 {
    match card {
        c if c.is_digit(10) => c.to_digit(10).unwrap() as u64,
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => {
            assert!(false);
            0
        }
    }
}

fn find_best_type(cards: [u64; 5]) -> HandType {
    let mut best_type = HandType::High;
    for j in 2..=14 {
        if j == 11 {
            continue;
        }
        let mut replaced_cards = cards.clone();
        for c in replaced_cards.iter_mut() {
            if *c == 1 {
                *c = j;
            }
        }
        let new_type = get_card_type(replaced_cards);
        if new_type > best_type {
            best_type = new_type;
        }
    }
    best_type
}

#[derive(std::cmp::PartialEq, std::cmp::Eq)]
struct HandJoker {
    cards: [u64; 5],
    bid: u64,
    hand_type: HandType,
}

fn str_to_hand_joker(hand_str: &str) -> HandJoker {
    let mut new_hand = HandJoker {
        cards: [0, 0, 0, 0, 0],
        bid: 0,
        hand_type: HandType::High,
    };
    let mut it = hand_str.split_ascii_whitespace();
    let cards_chars: Vec<u64> = it
        .next()
        .unwrap()
        .chars()
        .map(|c| card_to_power_joker(c))
        .collect();
    new_hand.cards = cards_chars.try_into().unwrap();
    new_hand.bid = u64::from_str_radix(it.next().unwrap(), 10).unwrap();

    let hand_type = find_best_type(new_hand.cards);
    new_hand.hand_type = hand_type;

    new_hand
}

fn read_cards_joker(input: &str) -> Vec<HandJoker> {
    input.lines().map(|l| str_to_hand_joker(l)).collect()
}


impl std::cmp::PartialOrd for HandJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let type_self = self.hand_type;
        let type_other = other.hand_type;
        if type_self < type_other {
            return Some(Ordering::Less);
        } else if type_self > type_other {
            return Some(Ordering::Greater);
        } else {
            // Compare cards one by one
            for (&c_s, &c_o) in self.cards.iter().zip(other.cards.iter()) {
                if c_s < c_o {
                    return Some(Ordering::Less);
                } else if c_s > c_o {
                    return Some(Ordering::Greater);
                }
            }
            // All equal
            assert!(false);
            return Some(Ordering::Equal);
        }
    }
}

impl std::cmp::Ord for HandJoker {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[allow(dead_code)]
fn print_hand_joker(hand: &HandJoker) {
    let cards: String = hand.cards.map(|p| power_to_card_joker(p)).iter().collect();
    println!("cards: [{}], type: {:?}, bid: {}", cards, hand.hand_type, hand.bid);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut hands = read_cards_joker(input);
    hands.sort();

    let winnings: u64 = hands.iter().enumerate().map(|(r, hand)| {
        ((r as u64) + 1) * hand.bid
    }).sum();

    Some(winnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_order() {
        let pair = Hand{cards: [2,3,2,4,3], bid: 967};
        let one = Hand{cards: [2,3,12,11,8], bid: 47};
        let res = pair.cmp(&one);
        assert!(res == Ordering::Greater);
    }
}
