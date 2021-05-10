// Andy Lee - 500163559
// Rust

#![allow(dead_code)]
#![allow(unused_assignments)]

fn main() {
    let hand1 = [(2, 'D'), (14, 'D'), (3, 'D'), (4, 'D'), (5, 'D')];
    let hand2 = [(9, 'S'), (6, 'S'), (8, 'S'), (7, 'S'), (5, 'S')];
    
    // preset hands
    let _ace_straight = [(2, 'H'),( 3, 'S'),( 14, 'S'),( 4, 'C'),( 5, 'C')];
    let _high_card = [(10, 'H'),( 2, 'S'),( 14, 'S'),( 4, 'D'),( 5, 'C')];
    let _pair = [(2, 'H'),( 2, 'S'),( 14, 'S'),( 4, 'C'),( 5, 'C')];
    let _high_pair = [(14, 'H'),( 2, 'S'),( 14, 'S'),( 4, 'C'),( 5, 'C')];
    let _two_pair = [(2, 'H'),( 2, 'S'),( 14, 'S'),( 14, 'C'),( 5, 'C')];
    let _two_pair2 = [(14, 'H'),( 14, 'S'),( 13, 'D'),( 13, 'C'),( 5, 'C')];
    let _straight = [(10, 'D'),( 13, 'S'),( 12, 'S'),( 11, 'D'),( 14, 'H')];
    let _straight2 = [(10, 'S'),( 13, 'D'),( 12, 'C'),( 11, 'H'),( 14, 'S')];
    let _three_kind = [(14, 'H'),( 14, 'S'),( 14, 'D'),( 13, 'C'),( 5, 'C')];
    let _full_house = [(14, 'H'),( 14, 'S'),( 14, 'D'),( 5, 'C'),( 5, 'C')];
    let _flush = [(5, 'C'),( 7, 'C'),( 9, 'C'),( 11, 'C'),( 12, 'C')];
    let _flush2 = [(5, 'S'),( 7, 'S'),( 9, 'S'),( 11, 'S'),( 13, 'S')];
    let _straight_flush = [(5, 'S'),( 7, 'S'),( 9, 'S'),( 6, 'S'),( 8, 'S')];
    let _straight_flush2 = [(14, 'D'),( 2, 'D'),( 3, 'D'),( 5, 'D'),( 4, 'D')];
    let _royal_flush = [(14, 'S'),( 13, 'S'),( 11, 'S'),( 12, 'S'),( 10, 'S')];

    let _xxx = find_winner(&hand1,&hand2);
    let _xxx2 = find_winner(&_ace_straight,&_high_card);
    let _xxx3 = find_winner(&_pair,&_high_pair);
    let _xxx4 = find_winner(&_two_pair2,&_high_pair);
    let _xxx5 = find_winner(&_royal_flush,&_flush2);
    let _xxx6 = find_winner(&_full_house,&_three_kind);
    let _xxx7 = find_winner(&_straight2,&_straight_flush2);

}

fn find_winner(hand1:&[(u8,char);5],hand2:&[(u8,char);5]) {
    //println!("Comparing...\nHand 1: {:?}\nHand 2: {:?}\n",hand1,hand2);

    let v_hand1 = *hand1;
    let v_hand2 = *hand2;
    let h1 = calculate_score(cards_to_nums(v_hand1));
    let h2 = calculate_score(cards_to_nums(v_hand2));
    

    if h1 > h2 {
        println!("Winner hand 1: {:?}\nLosing hand 2: {:?}\n----------------------------------------------------", hand1, hand2);
    } else {
        println!("Winner hand 2: {:?}\nLosing hand 1: {:?}\n----------------------------------------------------", hand2, hand1);
    }
}

fn cards_to_nums(arr:[(u8,char);5] ) -> Vec<u8> {
    // converts a hand of 5 cards to numbers, returning vector.
    let mut sum = Vec::new();
    for i in arr.iter() {
        sum.push(card_to_num(&i));
    }
    sum
}

fn card_to_num((rank,suit): &(u8,char)) -> u8 {    
    // convert a single tuple of rank and suit into respective number
    match suit {
        'D' => 0*13 + rank - 2,
        'C' => 1*13 + rank - 2,
        'H' => 2*13 + rank - 2,
        'S' => 3*13 + rank - 2,
        _ => 0,
    }
}

fn calculate_score(hand:Vec<u8>) -> u32 {
    // calculates score based on hand patterns
    let card_rank = isolate_rank(hand.to_owned());
    let card_suit = isolate_suit(hand.to_owned());
    let first_card = card_rank[0];
    let reduced_hand:Vec<_> = card_rank.to_owned().into_iter()
                           .map(|x| x-first_card)
                           .collect();
    let mut score:u32 = 0;    
    let first_suit = card_suit[0];
    let same_suit = card_suit.iter().all(|&x| x==first_suit);        

    if same_suit  {
            // same suit matching
            if starts_with(&card_rank,[8,9,10,11,12]) {
                // royal flush
                score = card_suit[0] as u32 + 10000000;

            } else if starts_with(&reduced_hand,[0,1,2,3,12]) {
                // ace lead straight flush
                score = card_suit[0] as u32 + 9000000;

            } else if starts_with(&reduced_hand,[0,1,2,3,4]) {
                // non ace lead straight flush
                score = high_card(&hand) as u32 + 9500000;

            } else {
                // normal flush
                score = card_rank[4] as u32*10 + 6000000;
            }
    } else {
        // not suit matching
        if find_same_kinds(&card_rank,4).0 == 4 {
            // four of a kind
            score = find_same_kinds(&card_rank,4).1 as u32 + 8000000;

        } else if find_same_kinds(&card_rank,3).0 == 3 {
            // 3 of a kind or full house
            
            let triple_card = find_same_kinds(&card_rank,3).1;
            let result:Vec<_> = card_rank.into_iter().filter(|&x| x != triple_card).collect();
            if result[0] == result[1] {
                // full house
                score = triple_card as u32 + 7000000;
            } else {
                // 3 of a kind
                score = triple_card as u32 + 4000000;
            }
            

        } else if find_same_kinds(&card_rank,2).0 == 2 {
            // at least a pair
            let first_pair = find_same_kinds(&card_rank,2).1;
            let result:Vec<_> = card_rank.into_iter().filter(|&x| x != first_pair).collect();
            let second_pair = find_same_kinds(&result,2);
            //println!("{:?}",second_pair);

            if second_pair.0 == 2 {
                // 2 2-pairs
                score = first_pair as u32*10 + high_suit(&hand,first_pair) as u32 +
                        (second_pair.1 as u32*10 + high_suit(&hand,second_pair.1) as u32)*1000 + 3000000;
                
            } else {
                // 1 2-pairs
                score = first_pair as u32 * 10 + high_suit(&hand,second_pair.1) as u32 + 2000000;
            }


        } else if starts_with(&reduced_hand,[0,1,2,3,12]) {
            // straight_ace_lead
            score = high_card(&hand) as u32 + 5000000;

        } else if starts_with(&reduced_hand,[0,1,2,3,4]) {
            // straight_non_ace_lend
            score = high_card(&hand) as u32 + 5100000;

        } else {
            // high card
            score = high_card(&hand) as u32;
        }
    };

    //println!("SCORE: {}\n=========================",score);
    score
}

// finding pairs and kinds
fn find_same_kinds(hand:&Vec<u8>, kind:usize)-> (usize,u8) {
    // function for finding same rank cards based on kind = 2, 3 or 4
    //println!("Searching for {} kinds...",kind);
    let mut a = hand.to_vec();
    let limit = kind;
    a.sort();
    //println!("Searching {:?} for {:?} kinds...",a,kind);
    let mut result = (0, 0);
    loop {
        if a.len() == 0 || a.len() < limit {
            result = (0,0);
            break;
        }
        let val = a.remove(0);
        //println!("list: {:?}, popping: {} ", a, val);
        match kind {
            4 => if val == a[0] && val == a[1] && val == a[2] {                
                //println!("found match 4 of a kind!");
                result = (kind,val);
                break;
            },
            3 => if val == a[0] && val == a[1] {
                //println!("found match 3 of a kind!");
                result = (kind,val);
                break;
            },
            2 => if val == a[0] {
                //println!("found match 2 pair");
                result = (kind,val);
                break;
            },
            _ => {}
        }
    }
    //println!("Same kind results {:?}", result);
    result
}

fn starts_with(src:&Vec<u8>, target:[u8;5]) -> bool {
    // function to compare a hand with a specified pattern
    let mut flag = true;
    for i in 0..5 {
        if src[i] != target[i] && target[i] != 100 {
            flag = false;
        }
    }
    flag
}

fn high_card(hand:&Vec<u8>) -> u8 {
    let a = hand.to_vec();
    let mut result:Vec<_> = a.into_iter()
                            .map(|x| x%13*10 + (x/13))
                            .collect();
    
    result.sort();    
    //println!(" - High Card: {:?}",result[result.len()-1]);
    result[result.len()-1]
}

fn high_suit(hand:&Vec<u8>, rank:u8) -> u8 {
    // high suit in a hand

    let a = hand.to_vec();
    //println!("{:?} of rank {}",a,rank );
    let mut result:Vec<_> = a.into_iter()
            .filter(|&x| (x+13-rank)%13 == 0)
            .collect();
    result.sort();
    //println!("the result {:?}",result );

    let r = &result[result.len()-1..];
    //println!("Highest suit of Rank {} is: {:?}",rank,(isolate_suit(r.to_vec())[0]));
    isolate_suit(r.to_vec())[0]
}

fn isolate_rank(hand:Vec<u8>) -> Vec<u8> {
    // isolate rank in a hand

    let mut result = Vec::new();
    for i in hand.iter() {
        result.push(i%13);
    }

    result.sort();
    result    
}

fn isolate_suit(hand:Vec<u8>) -> Vec<u8> {
    // ioslate suit in a hand

    let mut result = Vec::new();
    for i in hand.iter() {
        result.push(i/13);
    }

    result.sort();
    result    
}

