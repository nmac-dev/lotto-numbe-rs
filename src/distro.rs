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


/// Tests Thunderball number distribution against historical lottery draws.
///
/// Evaluates simulated run counts against geometric distribution thresholds
/// (50% confidence average probability and 95% confidence interval).
pub fn test_thunderball_distro() {

    let tb_ave_prob: u64 = 8060598;  // geometric distribution (w/ 50% confidence)
    let tb_end_prob: u64 = 24150000; // 95% confidence interval

    let draws: [[u64; 6]; 102] = [
        // September 2026
        [19, 24, 29, 33, 36, 4 ],  // Wed 30 Sep 2026
        [1,  17, 25, 28, 31, 9 ],  // Tue 29 Sep 2026
        [4,  7,  12, 17, 37, 11],  // Sat 26 Sep 2026
        [12, 21, 26, 27, 37, 3 ],  // Fri 25 Sep 2026
        [8,  18, 21, 24, 34, 1 ],  // Wed 23 Sep 2026
        [1,  10, 18, 32, 34, 2 ],  // Tue 22 Sep 2026
        [2,  15, 16, 29, 30, 9 ],  // Sat 19 Sep 2026 {*}
        [2,  4,  6,  8,  17, 11],  // Fri 18 Sep 2026 {*}
        [6,  9,  16, 20, 36, 6 ],  // Wed 16 Sep 2026
        [5,  12, 25, 29, 39, 4 ],  // Tue 15 Sep 2026
        [5,  11, 16, 20, 38, 2 ],  // Sat 12 Sep 2026 {*}
        [3,  7,  17, 25, 28, 14],  // Fri 11 Sep 2026
        [1,  19, 21, 22, 37, 14],  // Wed 09 Sep 2026
        [14, 22, 23, 32, 35, 3 ],  // Tue 08 Sep 2026
        [8,  10, 22, 34, 38, 4 ],  // Sat 05 Sep 2026
        [13, 16, 17, 21, 26, 6 ],  // Fri 04 Sep 2026
        [8,  15, 27, 30, 36, 4 ],  // Wed 02 Sep 2026 {*}
        [3,  21, 23, 37, 39, 7 ],  // Tue 01 Sep 2026

        // August 2026
        [7,  12, 18, 19, 20, 11],  // Sat 29 Aug 2026 {*}
        [6,  9,  10, 28, 34, 14],  // Fri 28 Aug 2026 {*}
        [7,  22, 26, 33, 37, 7 ],  // Wed 26 Aug 2026
        [6,  8,  9,  27, 31, 13],  // Tue 25 Aug 2026
        [2,  13, 18, 28, 31, 2 ],  // Sat 22 Aug 2026 {*}
        [8,  9,  16, 37, 38, 6 ],  // Fri 21 Aug 2026
        [2,  3,  11, 12, 24, 6 ],  // Wed 19 Aug 2026
        [1,  3,  9,  15, 23, 12],  // Tue 18 Aug 2026
        [11, 16, 24, 34, 35, 3 ],  // Sat 15 Aug 2026
        [4,  15, 25, 27, 32, 11],  // Fri 14 Aug 2026
        [5,  7,  27, 28, 38, 12],  // Wed 12 Aug 2026
        [3,  10, 20, 25, 28, 7 ],  // Tue 11 Aug 2026
        [11, 16, 25, 32, 35, 2 ],  // Sat 08 Aug 2026
        [4,  18, 28, 30, 34, 11],  // Fri 07 Aug 2026
        [12, 16, 20, 21, 37, 2 ],  // Wed 05 Aug 2026
        [15, 24, 30, 35, 37, 8 ],  // Tue 04 Aug 2026
        [15, 31, 32, 34, 37, 8 ],  // Sat 01 Aug 2026

        // July 2026
        [1,  16, 19, 21, 27, 12],  // Fri 31 Jul 2026 {*}
        [23, 26, 28, 32, 34, 13],  // Wed 29 Jul 2026
        [1,  7,  33, 34, 36, 14],  // Tue 28 Jul 2026 {*}
        [13, 15, 17, 21, 23, 14],  // Sat 25 Jul 2026 {*}
        [10, 16, 27, 29, 34, 3 ],  // Fri 24 Jul 2026
        [3,  9,  29, 30, 32, 1 ],  // Wed 22 Jul 2026
        [19, 27, 30, 32, 36, 3 ],  // Tue 21 Jul 2026
        [3,  6,  19, 25, 36, 6 ],  // Sat 18 Jul 2026
        [5,  8,  20, 24, 25, 1 ],  // Fri 17 Jul 2026 {*}
        [8,  14, 17, 18, 34, 3 ],  // Wed 15 Jul 2026
        [1,  15, 24, 25, 36, 7 ],  // Tue 14 Jul 2026
        [14, 16, 18, 23, 35, 7 ],  // Sat 11 Jul 2026
        [1,  8,  11, 23, 26, 9 ],  // Fri 10 Jul 2026
        [13, 18, 24, 25, 34, 2 ],  // Wed 08 Jul 2026
        [11, 12, 21, 33, 38, 10],  // Tue 07 Jul 2026
        [2,  5,  11, 25, 26, 4 ],  // Sat 04 Jul 2026
        [15, 17, 20, 24, 31, 13],  // Fri 03 Jul 2026
        [3,  11, 19, 28, 31, 3 ],  // Wed 01 Jul 2026 {*}

        // June 2026
        [5,  23, 31, 32, 35, 10],  // Tue 30 Jun 2026
        [16, 22, 30, 31, 35, 12],  // Sat 27 Jun 2026
        [2,  7,  16, 28, 30, 14],  // Fri 26 Jun 2026
        [2,  15, 24, 25, 38, 13],  // Wed 24 Jun 2026
        [4,  9,  12, 29, 32, 3 ],  // Tue 23 Jun 2026
        [6,  8,  9,  16, 23, 14],  // Sat 20 Jun 2026
        [7,  19, 23, 24, 38, 3 ],  // Fri 19 Jun 2026
        [13, 16, 23, 25, 26, 4 ],  // Wed 17 Jun 2026
        [8,  16, 29, 36, 39, 11],  // Tue 16 Jun 2026
        [15, 18, 21, 27, 39, 4 ],  // Sat 13 Jun 2026
        [21, 24, 29, 31, 36, 6 ],  // Fri 12 Jun 2026
        [1,  8,  12, 14, 24, 10],  // Wed 10 Jun 2026
        [1,  16, 29, 33, 39, 1 ],  // Tue 09 Jun 2026
        [6,  11, 25, 29, 37, 6 ],  // Sat 06 Jun 2026
        [2,  4,  9,  33, 37, 1 ],  // Fri 05 Jun 2026
        [7,  22, 25, 30, 37, 10],  // Wed 03 Jun 2026
        [2,  11, 33, 37, 38, 10],  // Tue 02 Jun 2026

        // May 2026
        [5,  20, 24, 28, 39, 2 ],  // Sat 30 May 2026
        [7,  13, 19, 35, 39, 12],  // Fri 29 May 2026 {*}
        [4,  12, 20, 26, 36, 13],  // Wed 27 May 2026
        [4,  17, 24, 27, 37, 6 ],  // Tue 26 May 2026 {*}
        [14, 16, 23, 26, 35, 1 ],  // Sat 23 May 2026 {*}
        [2,  9,  13, 20, 24, 8 ],  // Fri 22 May 2026 {*}
        [1,  15, 23, 25, 30, 2 ],  // Wed 20 May 2026
        [5,  6,  15, 26, 39, 1 ],  // Tue 19 May 2026
        [16, 17, 20, 26, 29, 5 ],  // Sat 16 May 2026
        [8,  15, 26, 29, 30, 13],  // Fri 15 May 2026
        [10, 11, 18, 28, 29, 2 ],  // Wed 13 May 2026
        [2,  3,  4,  14, 19, 11],  // Tue 12 May 2026
        [1,  3,  26, 27, 38, 12],  // Sat 09 May 2026
        [11, 14, 22, 31, 32, 3 ],  // Fri 08 May 2026
        [2,  12, 19, 24, 39, 12],  // Wed 06 May 2026
        [1,  2,  21, 33, 37, 9 ],  // Tue 05 May 2026
        [8,  12, 24, 28, 35, 12],  // Sat 02 May 2026
        [3,  12, 22, 34, 35, 8 ],  // Fri 01 May 2026

        // April 2026
        [1,  23, 26, 29, 37, 13],  // Wed 29 Apr 2026
        [4,  12, 20, 35, 37, 10],  // Tue 28 Apr 2026
        [10, 16, 19, 22, 35, 4 ],  // Sat 25 Apr 2026
        [3,  4,  23, 28, 31, 4 ],  // Fri 24 Apr 2026
        [11, 18, 23, 30, 33, 11],  // Wed 22 Apr 2026
        [3,  7,  24, 31, 35, 2 ],  // Tue 21 Apr 2026
        [2,  9,  15, 28, 33, 5 ],  // Sat 18 Apr 2026
        [1,  8,  9,  23, 38, 14],  // Fri 17 Apr 2026
        [1,  5,  17, 34, 35, 8 ],  // Wed 15 Apr 2026
        [7,  16, 24, 31, 39, 6 ],  // Tue 14 Apr 2026 {*}
        [6,  10, 11, 29, 38, 12],  // Sat 11 Apr 2026
        [11, 14, 16, 26, 31, 14],  // Fri 10 Apr 2026
        [7,  10, 17, 31, 33, 11],  // Wed 08 Apr 2026 {*}
        [1,  2,  5,  7,  35, 7 ],  // Tue 07 Apr 2026
    ];

    let mut ave_probs: u32 = 0;
    let mut end_probs: u32 = 0;
    let mut failures:  u32 = 0;

    for target in draws {
        print!("Lotto-Target: {:?}\n", target);

        let runs = runs_to_target(&target, 5, 39, 1, 14, true);

        let mut result: &str;
        if      runs < tb_ave_prob {
            result = "\x1b[32m[PASSED++]\x1b[0m";
            ave_probs += 1;
        }
        else if runs < tb_end_prob {
            result = "\x1b[33m[PASSED]\x1b[0m";
            end_probs += 1;
        }
        else
        {
            result = "\x1b[31m[FAILED]\x1b[0m";
            failures += 1;
        }

        print!("{} {}/{}:{}\n", result, runs, tb_ave_prob, tb_end_prob);
    }

    print!("PASSED++: {} | PASSED: {} | FAILED: {}", ave_probs, end_probs, failures);
}
