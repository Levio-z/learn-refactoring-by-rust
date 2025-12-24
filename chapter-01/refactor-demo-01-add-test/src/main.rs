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

fn statement(invoice: &Invoice, plays: &HashMap<String, Play>) -> String {
    let mut total_amount = 0;
    let mut volume_credits = 0;
    let mut result = format!("Statement for {}\n", invoice.customer);
    let format_usd = |amount: f64| -> String { format!("${:.2}", amount) };
    for perf in &invoice.performances {
        let play = plays.get(&perf.play_id).expect("Play not found");

        let mut this_amount;

        match play.kind.as_str() {
            "tragedy" => {
                this_amount = 40000;
                if perf.audience > 30 {
                    this_amount += 1000 * (perf.audience - 30);
                }
            }
            "comedy" => {
                this_amount = 30000;
                if perf.audience > 20 {
                    this_amount += 10000 + 500 * (perf.audience - 20);
                }
                this_amount += 300 * perf.audience;
            }
            _ => panic!("unknown type: {}", play.kind),
        }

        // Calculate volume credits
        volume_credits += std::cmp::max(perf.audience as i32 - 30, 0) as u32;
        if play.kind == "comedy" {
            volume_credits += perf.audience / 5;
        }

        // Print line for this performance
        result += &format!(
            " {}: {} ({} seats)\n",
            play.name,
            format_usd(this_amount as f64 / 100.0),
            perf.audience
        );

        total_amount += this_amount;
    }

    result += &format!(
        "Amount owed is {}\n",
        format_usd(total_amount as f64 / 100.0)
    );
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
