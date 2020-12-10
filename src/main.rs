use anyhow::{bail, Context};
use itertools::{self, Itertools};
use std::collections::{HashMap, HashSet};
use std::fs;

type Program<'a> = Vec<(&'a str, i64)>;

fn main() -> anyhow::Result<()> {
    let f = fs::read_to_string("input")?;

    let lines = f.lines();

    // Any adapter can take input of 1, 2, 3 jolts lower than output.
    let mut output_joltages = lines.map(str::parse::<i64>).map(Result::unwrap).collect::<HashSet<i64>>();

    // Add built-in
    let builtin_output = output_joltages.iter().max().unwrap() + 3;
    output_joltages.insert(builtin_output);

    let mut current_joltage = 0;

    let mut differences = Vec::with_capacity(output_joltages.len());
    let mut out_joltages = output_joltages.iter().cloned().collect::<Vec<i64>>();
    out_joltages.sort();

    for &next in out_joltages.iter() {
        let diff = next - current_joltage;
        differences.push(diff);

        current_joltage = next;
    }

    let num_diff_of_one = differences.iter().filter(|&&x| x == 1).count();
    let num_diff_of_three = differences.iter().filter(|&&x| x == 3).count();

    println!("{}", num_diff_of_one * num_diff_of_three);

    Ok(())
}
