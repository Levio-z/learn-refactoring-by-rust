use std::{collections::HashMap, fs};

use serde::Deserialize;
mod create_statement_data;
use create_statement_data::{StatementData, create_statement_data};

#[derive(Debug, Deserialize, Clone, Default)]
pub(crate) struct Play {
    name: String,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Performance {
    play_id: String,
    audience: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Invoice {
    customer: String,
    performances: Vec<Performance>,
}

fn usd(amount: u32) -> String {
    format!("${:.2}", amount as f64 / 100.0)
}

fn statement(invoice: &Invoice, plays: &HashMap<String, Play>) -> String {
    render_plain_text(&create_statement_data::create_statement_data(
        invoice, plays,
    ))
}

fn render_plain_text(statement_data: &StatementData) -> String {
    let mut result = format!("Statement for {}\n", statement_data.customer);
    for perf in &statement_data.performances {
        // Print line for this performance
        result += &format!(
            " {}: {} ({} seats)\n",
            perf.play.name,
            usd(perf.amount),
            perf.audience
        );
    }

    result += &format!("Amount owed is {}\n", usd(statement_data.total_amount));
    result += &format!(
        "You earned {} credits\n",
        statement_data.total_volume_credits
    );
    result
}
#[allow(unused)]
pub(crate) fn html_statement(invoice: &Invoice, plays: &HashMap<String, Play>) -> String {
    render_html(&create_statement_data(invoice, plays))
}
#[allow(unused)]
fn render_html(data: &StatementData) -> String {
    let mut result = String::new();

    result.push_str(&format!("<h1>Statement for {}</h1>\n", data.customer));
    result.push_str("<table>\n");
    result.push_str("<tr><th>play</th><th>seats</th><th>cost</th></tr>");

    for perf in &data.performances {
        result.push_str(&format!(
            " <tr><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            perf.play.name,
            perf.audience,
            usd(perf.amount),
        ));
    }

    result.push_str("</table>\n");
    result.push_str(&format!(
        "<p>Amount owed is <em>{}</em></p>\n",
        usd(data.total_amount)
    ));
    result.push_str(&format!(
        "<p>You earned <em>{}</em> credits</p>\n",
        data.total_volume_credits
    ));

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read JSON files
    let plays_data =
        fs::read_to_string("chapter-01/plays.json").expect("Failed to read plays.json");
    let invoices_data =
        fs::read_to_string("chapter-01/invoices.json").expect("Failed to read invoices.json");

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
