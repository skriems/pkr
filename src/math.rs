//! Poker Math
//!
//! # Preliminary Considerations
//!
//! - number of starting hands: 169
//!   - 13 pocket pairs
//!   - 78 suited hands
//!   - 78 offsuited hands (excl. pocket pairs)
//!
//! - number of possible combinations
//!   - 52! / (2! * 50!) = 1326 (binominal coefficient)
//!
//! # Combinatorics
//!
//! - 16 combos of unpaired hands
//! - 12 combos unpaired offsuit hands
//! -  4 combos unpaired suited hands
//! -  6 combos pocket pairs (3 if card is on board)
//!
//! ## binominal coefficient
//!
//! `k` cards out of `n`: n! / (k! (n-k)!)
//!
//!
//! # Probabilities
//!
//!
//! # Pot Odds
//!
//! 2$ into 10$ Pot = 5:1
//! 5$ into 10$ Pot = 2:1
//!
//! # Equity
//!
//! Turn:   outs * 4 || outs * 3 + 8 (if outs > 8)
//! River:  outs * 2 || outs + 8
