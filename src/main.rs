extern crate core;

use std::collections::HashMap;
use std::io;
use std::string::ToString;
use rand::thread_rng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use strum_macros::Display;

static mut PLAYING: bool = true;

#[derive(Copy, Clone, Debug, EnumIter, Display)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Copy, Clone, Debug, EnumIter, PartialEq, Display)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

fn main() {
    println!("Welcome to Blackjack!");

    loop {
        let mut deck = Deck {
            cards: vec![]
        };
        deck.new_deck();
        deck.shuffle();

        let mut player_hand = Hand {
            cards: vec![],
            value: 0,
            aces: 0,
        };
        player_hand.add_card(deck.deal());
        player_hand.add_card(deck.deal());

        let mut dealer_hand = Hand {
            cards: vec![],
            value: 0,
            aces: 0,
        };
        dealer_hand.add_card(deck.deal());
        dealer_hand.add_card(deck.deal());

        // Set up the player's chips
        let mut player_chips = Chips {
            total: 100,
            bet: 0,
        };

        // Prompt the player for their bet
        take_bet(&mut player_chips);

        // Show cards (but keep one dealer card hidden)
        show_some(&player_hand, &dealer_hand);

        unsafe {
            while PLAYING {
                // Prompt for player to hit or stand
                unsafe { hit_or_stand(&mut deck, &mut player_hand); }

                // Show cards (but keep one dealer card hidden)
                show_some(&player_hand, &dealer_hand);

                // If player's hand exceeds 21, run player_busts() and break
                if player_hand.value > 21 {
                    player_busts(&mut player_chips);
                    break;
                }
            }
        }

        // If player hasn't busted, play dealer's hand until dealer reaches 17
        if player_hand.value <= 21 {
            while dealer_hand.value < 17 {
                hit(&mut deck, &mut dealer_hand);
            }

            // Show all cards
            show_all(&player_hand, &dealer_hand);

            // Run different winning scenarios
            if dealer_hand.value > 21 {
                dealer_busts(&mut player_chips);
            } else if dealer_hand.value > player_hand.value {
                dealer_wins(&mut player_chips);
            } else if dealer_hand.value < player_hand.value {
                player_wins(&mut player_chips);
            } else {
                push();
            }
        }

        // Inform player of their chips total
        println!("\nPlayer's winnings stand at {:?}", player_chips.total);

        // Ask to play again
        let mut line = String::new();
        println!("Would you like to play another hand? Enter 'y' or 'n'");
        io::stdin().read_line(&mut line).expect("Failed to read input.");

        if line.trim() == 'y'.to_string() {
            unsafe { PLAYING = true; }
            continue;
        } else {
            println!("Thanks for playing");
            break;
        }
    }
}

// CARD

#[derive(Copy, Clone, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub value: i32,
}

impl Card {
    fn set_value(&mut self) {
        match self.rank {
            Rank::Two => { self.value = 2; }
            Rank::Three => { self.value = 3; }
            Rank::Four => { self.value = 4 }
            Rank::Five => { self.value = 5 }
            Rank::Six => { self.value = 6 }
            Rank::Seven => { self.value = 7 }
            Rank::Eight => { self.value = 8 }
            Rank::Nine => { self.value = 9 }
            Rank::Ten => { self.value = 10 }
            Rank::Jack => { self.value = 10 }
            Rank::Queen => { self.value = 10 }
            Rank::King => { self.value = 10 }
            Rank::Ace => { self.value = 11 }
        }
    }
}

// DECK

#[derive(Clone, Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    fn new_deck(&mut self) {
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                // Create new Card
                let mut card = Card {
                    suit,
                    rank,
                    value: 0,
                };
                card.set_value();
                self.cards.push(card);
            }
        }
    }

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    fn deal(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}

// HAND

#[derive(Clone, Debug)]
pub struct Hand {
    cards: Vec<Card>,
    value: i32,
    aces: i32,
}

impl Hand {
    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
        self.value += card.value;
        if card.rank == Rank::Ace {
            self.aces += 1;
        }
    }

    fn adjust_for_aces(&mut self) {
        if self.value > 21 && self.aces > 0 {
            self.value -= 10;
            self.aces -= 1;
        }
    }
}

// CHIPS

#[derive(Clone, Debug)]
pub struct Chips {
    pub total: i32,
    pub bet: i32,
}

impl Chips {
    fn win_bet(&mut self) {
        self.total += self.bet;
    }

    fn lose_bet(&mut self) {
        self.total -= self.bet;
    }
}

fn take_bet(chips: &mut Chips) {
    loop {
        let mut line = String::new();
        println!("How many chips would you like to bet? ");
        io::stdin().read_line(&mut line).expect("Failed to read input.");

        let my_string = line.trim().to_string();
        let my_int = my_string.parse::<i32>().unwrap();

        if my_int > chips.total {
            println!("Sorry, your bet can't exceed {}", chips.total);
        } else {
            chips.bet = my_int;
            break;
        }
    }
}

fn hit(deck: &mut Deck, hand: &mut Hand) {
    hand.add_card(deck.deal());
    hand.adjust_for_aces();
}

unsafe fn hit_or_stand(deck: &mut Deck, hand: &mut Hand) {
    loop {
        let mut line = String::new();
        println!("Would you like to Hit or Stand? ('h' or 's') ");
        io::stdin().read_line(&mut line).expect("Failed to read input.");
        if line.trim() == "h".to_string() {
            hit(deck, hand);
        } else if line.trim() == "s".to_string() {
            println!("Player stands. Dealer is playing.");
            PLAYING = false;
            break;
        } else {
            println!("Please try again.");
            continue;
        }
        break;
    }
}


fn show_some(player: &Hand, dealer: &Hand) {
    println!("\nDealer's Hand:");
    println!(" <card hidden>");
    println!("{} of {}", dealer.cards[1].rank, dealer.cards[1].suit);
    println!("\nPlayer's Hand:");
    for card in &player.cards {
        println!("{} of {}", card.rank, card.suit);
    }
}

fn show_all(player: &Hand, dealer: &Hand) {
    println!("\nDealer's Hand:");
    for card in &dealer.cards {
        println!("{} of {}", card.rank, card.suit);
    }
    println!("\nPlayer's Hand:");
    for card in &player.cards {
        println!("{} of {}", card.rank, card.suit);
    }
}

fn player_busts(player: &mut Chips) {
    player.lose_bet();
}

fn player_wins(player: &mut Chips) {
    player.win_bet();
}

fn dealer_busts(player: &mut Chips) {
    player.win_bet();
}

fn dealer_wins(player: &mut Chips) {
    player.lose_bet();
}

fn push() {
    println!("Dealer and Player tie. It's a push.");
}
