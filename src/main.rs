use anyhow::{bail, Context};
use itertools::{self, Itertools};
use std::{fs};


fn main() -> anyhow::Result<()> {
    let f = fs::read_to_string("input")?;

    let lines = f.lines();

    // Any adapter can take input of 1, 2, 3 jolts lower than output.
    let mut output_joltages = lines.map(str::parse::<i64>).map(Result::unwrap).collect::<Vec<i64>>();

    // Add built-in
    let builtin_output = output_joltages.iter().max().unwrap() + 3;
    output_joltages.push(builtin_output);

    output_joltages.sort();

    // Known results, indexed by "number of adapters remaining", e.g. [0] == result if 0 adapters remaining
    let mut known_results = Vec::with_capacity(output_joltages.len());

    let combinations = count_combinations(0, builtin_output, &output_joltages, &mut known_results);

    println!("{}", combinations);

    Ok(())
}

// `remaining outputs` is always sorted
// `known_results` contains results indexed by "# of adapters remaining", e.g. [0] == result if 0 adapters remaining
fn count_combinations(input: i64, final_output: i64, remaining_outputs: &[i64], known_results: &mut Vec<i64>) -> i64 {
    // Correct & verified calculation (no off-by-one)
    if remaining_outputs.len() < known_results.len() {
        return known_results[remaining_outputs.len()];
    }

    if input == final_output {
        // We have just attached the built-in adapter
        return 1;
    }

    let applicable_outputs = remaining_outputs.iter()
        // These are sorted, so we can just look at the first 3
        .take(3)
        .filter(|&&out| out <= input + 3);

    let count = applicable_outputs.count();

    if count == 0 {
        0
    } else {
        let mut sum = 0;
        for idx in 0..count {
            let remaining = &remaining_outputs[idx + 1..];
            let combinations_for_remaining = count_combinations(remaining_outputs[idx], final_output, remaining, known_results);

            if known_results.len() <= remaining.len() {
                known_results.push(combinations_for_remaining);
            }
            sum += combinations_for_remaining;
        }
        sum
    }
}
