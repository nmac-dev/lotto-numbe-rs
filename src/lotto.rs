//! Generate lotto numbers using [`Rng`].

use crate::numbe::Rng;


/// Defines lottery game parameters and ticket generation behavior.
pub trait Ticket {
    /// Number of main numbers drawn for the lottery game.
    const MAIN_LEN:   usize;
    /// Maximum value of the main number pool (inclusive range `1..=MAIN_RANGE`).
    const MAIN_RANGE: u64;
    /// Number of special/bonus numbers drawn for the lottery game.
    const SP_LEN:     usize;
    /// Maximum value of the special/bonus number pool (inclusive range `1..=SP_RANGE`).
    const SP_RANGE:   u64;

    /// Generates lottery ticket lines formatted as a string according to game parameters.
    ///
    /// - `rows`: Number of ticket rows to generate.
    /// - Returns: A formatted [`String`] containing the generated ticket rows.
    fn create_tickets(rows: usize) -> String {
        create_tickets(
            rows,
            Self::MAIN_LEN,
            Self::MAIN_RANGE,
            Self::SP_LEN,
            Self::SP_RANGE,
        )
    }
}


/// UK Lotto configuration (6 numbers from 1 to 59).
pub struct Lotto;
impl Ticket for Lotto {
    const MAIN_LEN:   usize = 6;
    const MAIN_RANGE: u64   = 59;
    const SP_LEN:     usize = 0;
    const SP_RANGE:   u64   = 0;
}
impl Lotto {
    pub fn create_tickets(rows: usize) -> String {
        <Self as Ticket>::create_tickets(rows)
    }
}


/// Powerball lottery configuration (5 numbers from 1 to 69, plus 1 Powerball from 1 to 26).
pub struct Powerball;
impl Ticket for Powerball {
    const MAIN_LEN:   usize = 5;
    const MAIN_RANGE: u64   = 69;
    const SP_LEN:     usize = 1;
    const SP_RANGE:   u64   = 26;
}
impl Powerball {
    pub fn create_tickets(rows: usize) -> String {
        <Self as Ticket>::create_tickets(rows)
    }
}


/// EuroMillions lottery configuration (5 numbers from 1 to 50, plus 2 Lucky Stars from 1 to 12).
pub struct EuroMillions;
impl Ticket for EuroMillions {
    const MAIN_LEN:   usize = 5;
    const MAIN_RANGE: u64   = 50;
    const SP_LEN:     usize = 2;
    const SP_RANGE:   u64   = 12;
}
impl EuroMillions {
    pub fn create_tickets(rows: usize) -> String {
        <Self as Ticket>::create_tickets(rows)
    }
}


/// UK Thunderball lottery configuration (5 numbers from 1 to 39, plus 1 Thunderball from 1 to 14).
pub struct Thunderball;
impl Ticket for Thunderball {
    const MAIN_LEN:   usize = 5;
    const MAIN_RANGE: u64   = 39;
    const SP_LEN:     usize = 1;
    const SP_RANGE:   u64   = 14;
}
impl Thunderball {
    pub fn create_tickets(rows: usize) -> String {
        <Self as Ticket>::create_tickets(rows)
    }
}


/// UK Set For Life lottery configuration (5 numbers from 1 to 47, plus 1 Life Ball from 1 to 10).
pub struct SetForLife;
impl Ticket for SetForLife {
    const MAIN_LEN:   usize = 5;
    const MAIN_RANGE: u64   = 47;
    const SP_LEN:     usize = 1;
    const SP_RANGE:   u64   = 10;
}
impl SetForLife {
    pub fn create_tickets(rows: usize) -> String {
        <Self as Ticket>::create_tickets(rows)
    }
}


/// Generates main and optional special numbers for a single lottery ticket.
///
/// - `rng`:        PRNG instance used to generate numbers.
/// - `main_len`:   Count of main numbers to generate.
/// - `main_range`: Upper bound of the main number pool (inclusive).
/// - `sp_len`:     Count of special numbers to generate.
/// - `sp_range`:   Upper bound of the special number pool (inclusive).
/// - Returns: A `Vec<u64>` containing main numbers followed by any special numbers.
fn _gen_nums(rng: &mut Rng, main_len: usize, main_range: u64, sp_len: usize, sp_range: u64) -> Vec<u64> {

    let mut result: Vec<u64> = Vec::new();

    result.append(&mut rng.next_series(main_len, 1, main_range, true));
    if sp_len > 0 {
        result.append(&mut rng.next_series(sp_len, 1, sp_range, true));
    }

    result
}


/// Formats a series of numbers into a string, separating main and special numbers.
///
/// - `series`: Slice of numbers containing main numbers and any special numbers.
/// - `split`:  Index where the special numbers begin.
/// - Returns: A formatted [`String`] representation of the series.
fn _series_to_string(series: &[u64], split: usize) -> String {

    let mut result = String::new();

    result.push_str(&format!("{:?}", &series[0..(split)]));
    if split > 0 {
        result.push_str(&format!("\t::{:?}", &series[(split)..(series.len())]));
    }

    result
}


/// Generates multiple rows of lottery tickets with main and optional special numbers.
///
/// - `rows`: Number of ticket rows to generate.
/// - `ml`:   Count of main numbers per row.
/// - `mr`:   Upper bound of the main number pool (inclusive).
/// - `sl`:   Count of special numbers per row.
/// - `sr`:   Upper bound of the special number pool (inclusive).
/// - Returns: A formatted [`String`] containing the generated ticket rows.
pub fn create_tickets(rows: usize, ml: usize, mr: u64, sl: usize, sr: u64) -> String {

    let mut rng    = Rng::new();
    let mut result = String::new();

    for i in 0..rows {
        let main_series: Vec<u64> = rng.next_series(ml, 1, mr, true);

        result.push_str(&format!(
            "Row-{}: {:?}",
            i + 1,
            main_series,
        ));
        if sl > 0 {
            let sp_series: Vec<u64> = rng.next_series(sl, 1, sr, true);
            result.push_str(&format!(
                "\t::{:?}",
                sp_series
            ));
        }
        result.push('\n');
    }

    result
}


/// Sorts the main and special numbers of a series in place independently.
///
/// - `series`: Mutable slice of numbers containing main and special numbers.
/// - `split`:  Index where the special numbers begin.
fn _sort_series(series: &mut [u64], split: usize) {

    let (main, sp) = series.split_at_mut(split.min(series.len()));

    main.sort_unstable();
    sp.sort_unstable();
}


/// Prints sample tickets for all supported lottery games to standard output.
pub fn print_all_tickets() {
    println!("[Lotto]\n{}",        Lotto::create_tickets(1));
    println!("[Powerball]\n{}",    Powerball::create_tickets(1));
    println!("[EuroMillions]\n{}", EuroMillions::create_tickets(1));
    println!("[Thunderball]\n{}",  Thunderball::create_tickets(1));
    println!("[Set-for-Life]\n{}", SetForLife::create_tickets(1));
}
