// Libraries
use std::io;
use std::io::Write;
use rand::Rng;

// Card struct for named values and Card methods
//#[derive(Clone)]
struct Card{
    face: u8,
    suit: String,
}

// make_deck fills a passed vector with 52 Cards, the standard playing deck
fn make_deck(deck: &mut Vec<Card>) {
    // for each suit
    for s in 1..5{
        // for each face
        for f in 1..14{
	    let mut temp = Card{
            face: f,
            suit: String::new(),
        };
	    
        // Assign string based on suit
	    match s{
	        1 => temp.suit = String::from("Spades"),
	        2 => temp.suit = String::from("Clubs"),
	        3 => temp.suit = String::from("Diamonds"),
	        4 => temp.suit = String::from("Hearts"),
	        _ => println!("ERROR"),
        }
        // Add to the vector    
	    deck.push(temp);
        }
    }
}

fn sort(hand: &mut Vec<Card>){
    //sort hand by Card face
    for i in 1..hand.len(){
        for j in (1..i+1).rev(){
            if hand[j - 1].face <= hand[j].face {break;}
            hand.swap(j - 1, j)
        }
    }
}

// Display cards by name from a vector
fn display(hand: &mut Vec<Card>){
    sort(hand);
    for card in hand{
        match card.face{
	    1 => println!("Ace of {}", card.suit),
	    11 => println!("Jack of {}", card.suit),
	    12 => println!("Queen of {}", card.suit),
	    13 => println!("King of {}", card.suit),
	    _ => println!("{} of {}", card.face, card.suit),
        }
    }
} 

// Check if there are any sets of four faces for scoring and AI decision making
fn place_book(hand: &mut Vec<Card>) -> u8 {
    // Counters for each possible card face
    let mut count: [u8; 14] = [0; 14];
 
    // Count how many of each face are in the hand
    for card in hand.iter(){
        count[(card.face - 1) as usize ] += 1;
    }
    
    // Decision making, if a book is present remove the related Cards and return a point
    // For the AI if there are no books to place return a number to direct to ask for a card.
    if count[0] >= 1{
        if count[0] == 4{
            let mut index = 0;
            while index < hand.len(){
                if hand[index].face == 1{
                    hand.remove(index);
                }
                else{
                    index += 1;
                }
            }
            return 1;
        }
        else{
            2
        }
    }
    else if count[1] >= 1{
        if count[1] == 4{
            let mut index = 0;
            while index < hand.len(){
                if hand[index].face == 2{
                    hand.remove(index);
                }
                else{
                    index += 1;
                }
            }
            return 1;
        }
        else{
            3
        }
    }
    else if count[2] >= 1{
        if count[2] == 4{
            let mut index = 0;
            while index < hand.len(){
                if hand[index].face == 3{
                    hand.remove(index);
                }
                else{
                    index += 1;
                }
            }
            return 1;
        }
        else{
            4
        }
    }
    else if count[3] >= 1{
        if count[3] == 4{
            let mut index = 0;
            while index < hand.len(){
                if hand[index].face == 4{
                    hand.remove(index);
                }
                else{
                    index += 1;
                }
            }
            return 1;
        }
        else{
            5
        }
    }
    else if count[4] >= 1{
        if count[4] == 4{
            let mut index = 0;
            while index < hand.len(){
                if hand[index].face == 5{
                    hand.remove(index);
                }
                else{
                    index += 1;
                }
            }
            return 1;
        }
        else{
            6
        }
    }
    else if count[5] >= 1{
        if count[5] == 4{
            let mut index = 0;
            while index < hand.len(){
                if hand[index].face == 6{
                    hand.remove(index);
                }
                else{
                    index += 1;
                }
            }
            return 1;
        }
        else{
            7
        }
    }
    else if count[6] >= 1{
        if count[6] == 4{
            let mut index = 0;
            while index < hand.len(){
                if hand[index].face == 7{
                    hand.remove(index);
                }
                else{
                    index += 1;
                }
            }
            return 1;
        }
        else{
            8
        }
    }
    else if count[7] >= 1{
        if count[7] == 4{
            let mut index = 0;
            while index < hand.len(){
                if hand[index].face == 8{
                    hand.remove(index);
                }
                else{
                    index += 1;
                }
            }
            return 1;
        }
        else{
            9
        }
    }
    else if count[8] >= 1{
        if count[8] == 4{
            let mut index = 0;
            while index < hand.len(){
                if hand[index].face == 9{
                    hand.remove(index);
                }
                else{
                    index += 1;
                }
            }
            return 1;
        }
        else{
            10
        }
    }
    else if count[9] >= 1{
        if count[9] == 4{
            let mut index = 0;
            while index < hand.len(){
                if hand[index].face == 10{
                    hand.remove(index);
                }
                else{
                    index += 1;
                }
            }
            return 1;
        }
        else{
            11
        }
    }
    else if count[10] >= 1{
        if count[10] == 4{
            let mut index = 0;
            while index < hand.len(){
                if hand[index].face == 11{
                    hand.remove(index);
                }
                else{
                    index += 1;
                }
            }
            return 1;
        }
        else{
            12
        }
    }
    else if count[11] >= 1{
        if count[11] == 4{
            let mut index = 0;
            while index < hand.len(){
                if hand[index].face == 12{
                    hand.remove(index);
                }
                else{
                    index += 1;
                }
            }
            return 1;
        }
        else{
            13
        }
    }
    else if count[12] >= 1{
        if count[12] == 4{
            let mut index = 0;
            while index < hand.len(){
                if hand[index].face == 13{
                    hand.remove(index);
                }
                else{
                    index += 1;
                }
            }
            return 1;
        }
        else{
            14
        }
    }
    else{
        0
    }
}

// Remove a random Card from a deck to add to a hand
fn draw(deck: &mut Vec<Card>, hand: &mut Vec<Card>) {
    if deck.len() > 0{
        let number = rand::thread_rng().gen_range(1..deck.len());
        let card = deck.remove(number); 
	    hand.push(card);
    }
}

// Remove matching Cards from one hand and add to the other hand
fn ask(deck: &mut Vec<Card>, hand: &mut Vec<Card>, number: u8) -> u8 {
    println!("Do you have any {}s?", number);
    let init = deck.len();
    if deck.len() > 0{
        let mut index = 0;
        while index < deck.len(){
            if deck[index].face == number{
                let card = deck.remove(index); 
	            hand.push(card);
            }
            else{
                index += 1;
            }
        }
    }
    if init == deck.len(){
        println!("Go Fish!");
        return 0;
    }
    else{
        1
    }
}

fn main() {
    // Fill a vector with deck of cards
    let mut deck: Vec<Card> = Vec::new();
    make_deck(&mut deck);

    // Randomly 'draw' 7 cards for computer hand and player hand
    let mut player_hand: Vec<Card> = Vec::new();
    let mut cpu_hand: Vec<Card> = Vec::new();
    for _i in 0..7{
        draw(&mut deck, &mut player_hand);
        draw(&mut deck, &mut cpu_hand);
    }

    let mut player_score = 0;
    let mut cpu_score = 0;

    // display player hand and wait for input
    display(&mut player_hand);

    println!("For a full list of commands, type help");
    // Input options: help (display the following)
    //                book # (take list and face to update score)
    // 		          ask # (check CPU list and return either card or call draw)
    //                show hand (print hand out)
    //                score (print the score for each player)

    
    let mut finished = false;
    while !finished{
        print!("> ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read input");
        
        if command.trim() == "quit"{
            finished = true;
            println!("Thank you for playing!");
            continue;
        }
        else if command.trim() == "help"{
            println!("Command Options:\nhelp: display the following\nbook: check and place 4 of a kind for points\n\
                      ask: ask the CPU for cards\nhand: show your current hand\nscore: show the current score");
            continue;
        }
        else if command.trim() == "book"{
            let score = place_book(&mut player_hand);
            if score == 1{
                player_score += score;
            }
            display(&mut player_hand);
            continue;
        }
        else if command.trim() == "ask"{
            print!("What card would you like to request? (Numbered 1-13)\n> ");
            io::stdout().flush().unwrap();

            let mut request = String::new();
            io::stdin()
                .read_line(&mut request)
                .expect("Failed to read input");
            let fish = ask(&mut cpu_hand, &mut player_hand, request.trim().parse::<u8>().unwrap());
            if fish == 0{
                draw(&mut deck, &mut player_hand);
            }
            display(&mut player_hand);
        }
        else if command.trim() == "hand"{
            display(&mut player_hand);
            continue;
        }
        else if command.trim() == "score"{
            println!("Player: {}\tCPU: {}", player_score, cpu_score);
            continue;
        }
        else{
            if deck.len() == 0{
                if player_hand.len() == 0{
                    finished = true;
                }
                else{}
            }
        }

        // CPU AI decision making
        let cpu_dec = place_book(&mut cpu_hand);
        if cpu_dec == 1{
            cpu_score += 1;
        }
        else if cpu_dec >= 2{
            let fish = ask(&mut player_hand, &mut cpu_hand, cpu_dec - 1);
            if fish == 0{
                draw(&mut deck, &mut cpu_hand);
            }
        } 
    }

    // With game over, determine winner
    if player_score > cpu_score{
        println!("You Win!");
    }
    else if player_score == cpu_score{
        println!("It's a Draw!");
    }
    else{
        println!("You Lose!");
    }
    
}