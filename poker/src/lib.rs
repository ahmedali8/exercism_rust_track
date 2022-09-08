use std::collections::BTreeMap;

#[derive(Debug)]
struct PokerHand<'a> {
    card: &'a str,
    rank: u32,
}

impl<'a> PokerHand<'a> {
    fn new(card: &'a str) -> Self {
        PokerHand { card, rank: PokerHand::rank(card) }
    }

    fn rank(card: &str) -> u32 {
        const STRAIGHT_FLUSH: u32 = 2000009000;
        const FOUR_KIND: u32 = 2000008000;
        const FULL_HOUSE: u32 = 2000007000;
        const FLUSH: u32 = 2000005000;
        const STRAIGHT: u32 = 2000004000;
        const THREE_KIND: u32 = 2000003000;
        const TWO_PAIRS: u32 = 2000002000;
        const ONE_PAIR: u32 = 2000001000;

        let suits = card
            .split_whitespace()
            .map(|chars| &chars[chars.len() - 1..] )
            .collect::<Vec<&str>>();
        println!("suits: {:?}", suits);
        
        let kinds = card
        .split_whitespace()
        .map(|chars| {
            println!("chars {:?}", chars);
            println!(
                "&chars[..chars.len() - 1] {:?}",
                &chars[..chars.len() - 1]
            );
            match &chars[..chars.len() - 1] {
                "J" => 11,
                "Q" => 12,
                "K" => 13,
                "A" => 14,
                n => n.parse().unwrap(),
            }
        })
        .fold(
            BTreeMap::<u32, u32>::new(),
            |mut dict, rank| {
                *dict.entry(rank).or_insert(0) += 1;
                dict
            }
        );
        println!("kinds: {:?}", kinds);

        let mut kind_and_count = kinds
            .iter()
            .map(|(&x, &y)| (x, y))
            .collect::<Vec<(u32, u32)>>();

        kind_and_count.sort_by(|x, y| y.1.cmp(&x.1));

        match kind_and_count.as_slice() {
            [(a,4),(b,1)] => FOUR_KIND + 10*a + b,
            [(a,3),(b,2)] => FULL_HOUSE + 10*a + b,
            [(a,3), (b,1),(c,1)] => THREE_KIND + 10*a + b + c,
            [(a,2),(b,2),(c,1)] => TWO_PAIRS + 10*a + 10*b + c,
            [(a,2),(b,1),(c,1),(d,1)] => ONE_PAIR + 10*a + b + c + d,    
            _ => {
                let kind_values = kinds.keys().cloned().collect::<Vec<u32>>();
                let is_same_suit = suits.windows(2).all( |p| p[0] == p[1] );
                let is_asc_order = kind_values.windows(2).all( |d| d[0]+1 == d[1] || d[1] == 14);
                let fake_last_ace = kind_values[4] == 14 && kind_values[3] != 13;
                let sum: u32 = kind_values.iter().sum();
                match (is_same_suit, is_asc_order, fake_last_ace) {
                    (true, true, false) => STRAIGHT_FLUSH + sum,
                    (true, true, true) => STRAIGHT_FLUSH + sum - 14,
                    (true, false, _) => FLUSH + sum,
                    (false, true, false) => STRAIGHT + sum,
                    (false, true, true) => STRAIGHT + sum - 14,
                    (false, false, _) => {
                        let s = kind_values.iter().rev().map( |d| d.to_string()).collect::<String>();
                        let parse: u32 = s.parse().unwrap();
                        parse
                    }
                }
            }
        }
    }
}


pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 0 {
        vec![]
    } else {
        let mut poker_hands = hands.iter().map( |h| PokerHand::new(h) ).collect::<Vec<PokerHand>>();
        poker_hands.sort_by( |h1, h2| h2.rank.cmp(&h1.rank) );
        poker_hands.iter().filter( |d| d.rank == poker_hands[0].rank).map(|d| d.card).collect()
    }
}
