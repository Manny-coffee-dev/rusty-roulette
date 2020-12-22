use rand::distributions::{Distribution, Uniform};

#[derive(Debug, PartialEq)]
enum OddEven {
    Odd,
    Even,
}

#[derive(Debug, PartialEq)]
enum RedBlack {
    Red,
    Black,
}

// TODO: Add the rest of the [bet types](https://en.wikipedia.org/wiki/Roulette#Types_of_bets)
#[derive(Debug, PartialEq)]
enum BetType {
    Single(i32),
    // Split,
    // Street,
    // Corner,
    // DoubleStreet,
    // Trio,
    Basket,
    LowPass,
    HighPass,
    RedorBlack(RedBlack),
    OddorEven(OddEven),
    DozenBet(i32),
    ColumnBet(i32),
}

#[derive(Debug, PartialEq)]
struct Bet {
    bet_type: BetType,
    bet_amount: f32,
}

fn main() {
    println!("--- Rusty Roulette Simulator ---");
    // Set up environment
    let wheel = Uniform::new(0, 36);
    let mut bets = Vec::<Bet>::new();
    // TODO: Add some user input for placing bets
    bets.push(Bet {
        bet_type: BetType::Single(1),
        bet_amount: 1.0,
    });
    bets.push(Bet {
      bet_type: BetType::Basket,
      bet_amount: 1.0,
    });
    bets.push(Bet {
      bet_type: BetType::LowPass,
      bet_amount: 1.0,
    });
    bets.push(Bet {
      bet_type: BetType::HighPass,
      bet_amount: 1.0,
    });
    bets.push(Bet {
        bet_type: BetType::RedorBlack(RedBlack::Red),
        bet_amount: 1.0,
    });
    bets.push(Bet {
        bet_type: BetType::OddorEven(OddEven::Even),
        bet_amount: 1.0,
    });
    bets.push(Bet {
      bet_type: BetType::Single(1),
      bet_amount: 1.0,
  });
  bets.push(Bet {
        bet_type: BetType::DozenBet(1),
        bet_amount: 1.0,
  });
  bets.push(Bet {
    bet_type: BetType::ColumnBet(1),
    bet_amount: 1.0,
  });

    // Spin the wheel and determine the outcome
    let number = spin(wheel);
    println!("It's a {0:?}!", number);

    // See if which bets won
    let mut total_winnings = 0.0;
    for bet in bets {
        total_winnings += results_handler(number, bet);
    }
    println!("You won £{0:?}", total_winnings);
}

// Check if a number is odd or even
fn number_odd_or_even(number: i32) -> OddEven {
    let odd_even;
    if number % 2 == 0 {
        odd_even = OddEven::Even;
    } else {
        odd_even = OddEven::Odd;
    }
    odd_even
}

// Check if a number is red or black
fn number_red_or_black(number: i32) -> RedBlack {
    let mut colour = RedBlack::Black;
    let red = [
        1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36,
    ];
    // TODO: Add error handling to check if number is not red or black!
    //let black = [2, 4, 6, 8, 10, 11,  13, 15, 17, 20, 22, 24,  26, 28, 29, 31, 33, 35 ];
    if red.contains(&number) {
        colour = RedBlack::Red;
    }
    // } else if black.contains(&number) {
    //   colour = RedBlack::Black;
    // }
    colour
}

// Generate a random number from our wheel
fn spin(wheel: Uniform<i32>) -> i32 {
    let mut rng = rand::thread_rng();
    wheel.sample(&mut rng)
}

// Handle the results of spin
fn results_handler(number: i32, bet: Bet) -> f32 {
    let mut winnings = 0.0;
    match bet.bet_type {
        BetType::Single(value) => winnings += single_handler(number, bet.bet_amount, value),
        BetType::Basket => winnings += basket_handler(number, bet.bet_amount),
        BetType::LowPass => winnings += low_pass_handler(number, bet.bet_amount),
        BetType::HighPass => winnings += high_pass_handler(number, bet.bet_amount),
        BetType::RedorBlack(colour) => {
            winnings += red_or_black_handler(number, bet.bet_amount, colour)
        }
        BetType::OddorEven(odd_even) => {
            winnings += odd_or_even_handler(number, bet.bet_amount, odd_even)
        }
        BetType::DozenBet(dozen) => winnings += dozen_handler(number, bet.bet_amount, dozen),
        BetType::ColumnBet(column) => winnings += column_handler(number, bet.bet_amount, column),
    }
    winnings
}

// Handle a single bet
fn single_handler(spin_number: i32, bet_amount: f32, bet_number: i32) -> f32 {
    let mut winnings = 0.0;
    if spin_number == bet_number {
        winnings = (bet_amount * 36.0) + bet_amount;
    }
    winnings
}

// Handle a basket bet
fn basket_handler(spin_number: i32, bet_amount: f32) -> f32 {
  let mut winnings = 0.0;
  let basket = [0, 1, 2, 3 ];
  if basket.contains(&spin_number) {
      winnings = (bet_amount * 8.0) + bet_amount;
  }
  winnings
}

// Handle a low pass bet
fn low_pass_handler(spin_number: i32, bet_amount: f32) -> f32 {
  let mut winnings = 0.0;
  let low_pass = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18];
  if low_pass.contains(&spin_number) {
      winnings = (bet_amount * 1.0) + bet_amount;
  }
  winnings
}

// Handle a high pass bet
fn high_pass_handler(spin_number: i32, bet_amount: f32) -> f32 {
  let mut winnings = 0.0;
  let high_pass = [19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36];
  if high_pass.contains(&spin_number) {
      winnings = (bet_amount * 1.0) + bet_amount;
  }
  winnings
}

// Handle a red/black bed
fn red_or_black_handler(spin_number: i32, bet_amount: f32, bet_colour: RedBlack) -> f32 {
    let mut winnings = 0.0;
    let spin_colour = number_red_or_black(spin_number);
    if spin_colour == bet_colour {
        winnings = (bet_amount * 1.0) + bet_amount;
    }
    winnings
}

// Handle an odd/even bet
fn odd_or_even_handler(spin_number: i32, bet_amount: f32, bet_odd_even: OddEven) -> f32 {
    let mut winnings = 0.0;
    let spin_odd_even = number_odd_or_even(spin_number);
    if spin_odd_even == bet_odd_even {
        winnings = (bet_amount * 1.0) + bet_amount;
    }
    winnings
}

// Handle a dozen bet
fn dozen_handler(spin_number: i32, bet_amount: f32, bet_column: i32) -> f32 {
  let mut winnings = 0.0;
  let dozens = [
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    [13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24],
    [25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36]
    ];
  if dozens[(bet_column - 1) as usize].contains(&spin_number) {
      winnings = (bet_amount * 2.0) + bet_amount;
  }
  winnings
}

// Handle a column bet
fn column_handler(spin_number: i32, bet_amount: f32, bet_column: i32) -> f32 {
  let mut winnings = 0.0;
  let columns = [
    [1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34],
    [2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35],
    [3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36]
    ];
  if columns[(bet_column - 1) as usize].contains(&spin_number) {
      winnings = (bet_amount * 2.0) + bet_amount;
  }
  winnings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_even() {
        let mut count = 0;
        let mut flag = OddEven::Even;
        while count < 100 {
            assert_eq!(number_odd_or_even(count), flag);
            count += 1;
            if flag == OddEven::Even {
                flag = OddEven::Odd;
            } else {
                flag = OddEven::Even;
            }
        }
    }
    // TODO: Add tests for all results handlers

    #[test]
    fn number_odd_even() {
        let mut count = 0;
        let mut flag = OddEven::Even;
        let bet_odd = OddEven::Odd;
        let bet_even = OddEven::Even;
        while count < 40 {
            // Assume we 'spin' and get count
            let odd_even = number_odd_or_even(count);
            count += 1;
            if flag == OddEven::Even {
                assert_eq!(bet_even, odd_even);
                assert_ne!(bet_odd, odd_even);
                flag = OddEven::Odd;
            } else {
                assert_eq!(bet_odd, odd_even);
                assert_ne!(bet_even, odd_even);
                flag = OddEven::Even;
            }
        }
    }
}
