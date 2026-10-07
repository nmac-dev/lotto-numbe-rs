//! Test lotto numbers distribution.

use lotto


/// Simulates ticket generations until matching a target series and counts the attempts.
///
/// - `target`:     Target lottery number series to match.
/// - `main_len`:   Count of main numbers in the series.
/// - `main_range`: Upper bound of the main number pool (inclusive).
/// - `sp_len`:     Count of special numbers in the series.
/// - `sp_range`:   Upper bound of the special number pool (inclusive).
/// - `print`:      If `true`, prints progress counter to stdout on each iteration.
/// - Returns: Total number of simulation runs required to match the target.
pub fn runs_to_target(target: &[u64], main_len: usize, main_range: u64, sp_len: usize, sp_range: u64, print: bool) -> u64 {

    let mut target = target.to_vec();
    _sort_series(&mut target, main_len);

    let mut rng = Rng::new();
    let mut run: u64 = 0;

    loop {
        run += 1;
        if print { print!("Runs: {}\r", run) }

        let mut series = _gen_nums(&mut rng, main_len, main_range, sp_len, sp_range);
        _sort_series(&mut series, main_len);

        if series == target { break }
    }

    run
}
