use std::{collections::HashMap, fs};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Play {
    name: String,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Debug, Deserialize)]
struct Performance {
    play_id: String,
    audience: u32,
}

#[derive(Debug, Deserialize)]
struct Invoice {
    customer: String,
    performances: Vec<Performance>,
}

fn amount_for(perf: &Performance, plays: &HashMap<String, Play>) -> u32 {
    let mut result;
    match play_for(perf, plays).kind.as_str() {
        "tragedy" => {
            result = 40000;
            if perf.audience > 30 {
                result += 1000 * (perf.audience - 30);
            }
        }
        "comedy" => {
            result = 30000;
            if perf.audience > 20 {
                result += 10000 + 500 * (perf.audience - 20);
            }
            result += 300 * perf.audience;
        }
        _ => panic!("unknown type: {}", play_for(perf, plays).kind),
    }
    result
}
fn play_for<'a>(perf: &Performance, plays: &'a HashMap<String, Play>) -> &'a Play {
    plays.get(&perf.play_id).expect("Play not found")
}
// 1. Extracted function for calculating volume credits.
fn volume_credits_for(perf: &Performance, plays: &HashMap<String, Play>) -> u32 {
    let mut result = 0;
    result += std::cmp::max(perf.audience as i32 - 30, 0) as u32;
    if play_for(perf, plays).kind == "comedy" {
        result += perf.audience / 5;
    }
    result
}
// 2. replace some of the temporary variables, and the simplest place to start
//    is the format variable.
// 3. reanme format_usd to usd and move it out of the statement function.
// 4. move the repeated “divide by 100” behavior into a function as well.
fn usd(amount: f64) -> String {
    format!("${:.2}", amount / 100.0)
}
fn statement(invoice: &Invoice, plays: &HashMap<String, Play>) -> String {
    let mut total_amount = 0;
    let mut volume_credits = 0;
    let mut result = format!("Statement for {}\n", invoice.customer);
    for perf in &invoice.performances {
        // Calculate volume credits
        volume_credits += volume_credits_for(perf, plays);

        // Print line for this performance
        result += &format!(
            " {}: {} ({} seats)\n",
            play_for(perf, plays).name,
            usd(amount_for(perf, plays) as f64),
            perf.audience
        );

        total_amount += amount_for(perf, plays);
    }

    result += &format!("Amount owed is {}\n", usd(total_amount as f64));
    result += &format!("You earned {} credits\n", volume_credits);
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read JSON files
    let plays_data = fs::read_to_string("chapter-01/plays.json")?;
    let invoices_data = fs::read_to_string("chapter-01/invoices.json")?;

    let plays: HashMap<String, Play> = serde_json::from_str(&plays_data)?;
    let invoices: Vec<Invoice> = serde_json::from_str(&invoices_data)?;

    // Print statements
    for invoice in &invoices {
        let output = statement(invoice, &plays);
        println!("{}", output);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_statement_output_from_files() {
        // Read data directly from JSON files
        let plays_data = fs::read_to_string("../plays.json").expect("Failed to read plays.json");
        let invoices_data =
            fs::read_to_string("../invoices.json").expect("Failed to read invoices.json");

        let plays: HashMap<String, Play> =
            serde_json::from_str(&plays_data).expect("Failed to parse plays.json");
        let invoices: Vec<Invoice> =
            serde_json::from_str(&invoices_data).expect("Failed to parse invoices.json");

        // Test the first invoice
        let invoice = &invoices[0];
        let result = statement(invoice, &plays);

        let expected_output = "Statement for BigCo\n Hamlet: $650.00 (55 seats)\n As You Like It: $580.00 (35 seats)\n Othello: $500.00 (40 seats)\nAmount owed is $1730.00\nYou earned 47 credits\n";

        assert_eq!(result, expected_output);
    }
}
