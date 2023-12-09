use itertools::enumerate;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub struct CamelCard {
    number: usize,
    pub character: char,
}

impl Eq for CamelCard {}

impl fmt::Debug for CamelHand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Hand")
            .field("it", &self.to_string())
            .field("type", &self.hand_type)
            .field("strength", &self.total_hand_strength)
            .finish()
    }
}
impl PartialEq<Self> for CamelCard {
    fn eq(&self, other: &Self) -> bool {
        self.character == other.character
    }
}

impl PartialOrd<Self> for CamelCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CamelCard {
    fn cmp(&self, other: &Self) -> Ordering {
        self.character.cmp(&other.character)
    }
}

const CAMEL_CARD_COUNT: usize = 13;
const CAMEL_CARDS: [char; CAMEL_CARD_COUNT] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const HAND_SIZE: usize = 5;
impl CamelCard {
    pub fn from_char(c: char) -> Option<CamelCard> {
        let number = CAMEL_CARDS.iter().position(|c2| c == *c2)?;
        let character = c;
        Some(CamelCard { number, character })
    }
}
#[derive(Clone)]
pub struct CamelHand {
    total_hand_strength: usize,
    hand: [CamelCard; HAND_SIZE],
    hand_type: u8,
    bid: usize,
}
impl Display for CamelHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let hand: String = self.hand.iter().map(|card| card.character).collect();
        write!(
            f,
            "hand {} of type {} with bid {}",
            hand, self.hand_type, self.bid
        )
    }
}

pub struct CamelHandSet {
    hands: Vec<CamelHand>,
}
impl CamelHandSet {
    pub fn total_winnings(&self) -> usize {
        let mut acc = 0;
        for (index, hand) in enumerate(&self.hands) {
            let rank = index + 1;
            let bid = hand.bid;
            acc += bid * rank;
        }
        acc
    }
    pub fn into_vec(self) -> Vec<CamelHand> {
        self.hands
    }
}
impl FromStr for CamelHandSet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hands: Vec<CamelHand> = s
            .trim()
            .lines()
            .map(|line| line.parse::<CamelHand>().ok())
            .collect::<Option<Vec<CamelHand>>>()
            .ok_or("Error collecting camelhand lines")?;
        hands.sort();
        Ok(Self { hands })
    }
}

impl PartialEq<Self> for CamelHand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl PartialOrd<Self> for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.total_hand_strength.cmp(&other.total_hand_strength))
    }
}
impl Eq for CamelHand {}
impl Ord for CamelHand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_hand_strength.cmp(&other.total_hand_strength)
    }
}
impl FromStr for CamelHand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("Parsing {}", s);
        // Has to be a line
        let (hand_token, bid_token): (&str, &str) = s
            .trim()
            .split_once(" ")
            .ok_or("Error splitting tokens".to_string())?;

        let bid = bid_token
            .trim()
            .parse()
            .map_err(|_| "Error parsing the bid".to_string())?;

        let hand_opts: [Option<CamelCard>; HAND_SIZE] = hand_token
            .trim()
            .chars()
            .next_chunk::<HAND_SIZE>()
            .map_err(|_| "Error getting elements from hand".to_string())?
            .map(|c| CamelCard::from_char(c));

        let mut count_map: [u8; CAMEL_CARD_COUNT] = [0; CAMEL_CARD_COUNT];

        let mut hand: [CamelCard; HAND_SIZE] = [CamelCard {
            number: 0,
            character: '2',
        }; HAND_SIZE];

        for i in 0..HAND_SIZE {
            let camel_card = hand_opts[i].ok_or("Error parsing camel card".to_string())?;
            count_map[camel_card.number] += 1;
            hand[i] = camel_card;
        }
        count_map.sort();
        let largest_count = count_map[CAMEL_CARD_COUNT - 1];
        let second_largest_count = count_map[CAMEL_CARD_COUNT - 2];

        let hand_type: u8 = match (largest_count, second_largest_count) {
            (5, _) => Ok(6), // Five of a kind
            (4, _) => Ok(5), //  Four of a kind
            (3, 2) => Ok(4), // Full house
            (3, 1) => Ok(3), // Three of a kind
            (2, 2) => Ok(2), // Two pair
            (2, 1) => Ok(1), // One pair
            (1, 1) => Ok(0), // High card
            (_, _) => Err("Unreachable state"),
        }?;

        let mut total_hand_strength = 0;
        for i in 0..HAND_SIZE {
            let card = &hand[i];
            let reverse_rank = HAND_SIZE - (i + 1); // 4 3 2 1 0
            let strength_to_add = card.number * (CAMEL_CARD_COUNT.pow(reverse_rank as u32));
            total_hand_strength += strength_to_add;
            print!("->{},", strength_to_add);
        }
        total_hand_strength += hand_type as usize * CAMEL_CARD_COUNT.pow(HAND_SIZE as u32);
        Ok(CamelHand {
            hand,
            hand_type,
            total_hand_strength,
            bid,
        })
    }
}
