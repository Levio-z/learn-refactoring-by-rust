use std::collections::HashMap;

use super::{Invoice, Performance, Play};
//  ===Split the phases (154)===
//  1.first apply Extract Function (106) to the code that makes up the second
// phase
//  2. create an intermediate object that represents the data structure needed
//     for rendering.
#[derive(Debug, Clone, Default)]
pub(crate) struct PerformanceData {
    pub play: Play,
    pub play_id: String,
    pub audience: u32,
    pub amount: u32,
    pub total_credits: u32,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct StatementData {
    pub customer: String,
    pub performances: Vec<PerformanceData>,
    pub total_amount: u32,
    pub total_volume_credits: u32,
}

// 2.1 Add the customer field to the intermediate object.(compile, test,
//    commit). 2.2 Move the performances field into the intermediate
//    object.(compile, test,    commit).
// 2.3 Populate each aPerformance with play data so the play name can be read
//    from the intermediate data (compile, test, commit).
// 2.4 Apply Move Function (198) to playFor and statement; update all references
//    in renderPlainText to use the new data (compile, test, commit).
// 2.5 Move the amountFor function using the same approach (compile, test,
//    commit).
// 2.6 Move the calculations for amount and volume credits (compile, test,
//    commit).
// 2.7 Replace Loop with Pipeline（231）.(compile, test,
//    commit).
// 2.8 Finally, update the statement function to use the new functions.(compile,
//    test, commit).
// 2.9 Create new file create_statement_data.rs to hold the compute
//    phase.(compile,    test, commit).

pub fn create_statement_data(invoice: &Invoice, plays: &HashMap<String, Play>) -> StatementData {
    let mut statement_data = StatementData {
        customer: invoice.customer.clone(),
        performances: invoice
            .performances
            .iter()
            .map(|perf| enrich_performance(perf, plays))
            .collect(),
        ..Default::default()
    };
    statement_data.total_amount = total_amount(&statement_data);
    statement_data.total_volume_credits = total_volume_credits(&statement_data);
    statement_data
}

fn play_for(perf: &PerformanceData, plays: &HashMap<String, Play>) -> Play {
    plays.get(&perf.play_id).expect("Play not found").clone()
}

fn amount_for(perf: &PerformanceData) -> u32 {
    let mut result;
    match perf.play.kind.as_str() {
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
        _ => panic!("unknown type: {}", perf.play.kind),
    }
    result
}

fn enrich_performance(perf: &Performance, plays: &HashMap<String, Play>) -> PerformanceData {
    let mut result = PerformanceData {
        play_id: perf.play_id.clone(),
        audience: perf.audience,
        ..Default::default()
    };
    result.play = play_for(&result, plays).clone();
    result.amount = amount_for(&result);
    result.total_credits = volume_credits_for(&result);
    result
}

fn volume_credits_for(perf: &PerformanceData) -> u32 {
    let mut result = 0;
    result += std::cmp::max(perf.audience as i32 - 30, 0) as u32;
    if perf.play.kind == "comedy" {
        result += perf.audience / 5;
    }
    result
}

// 2.7.1 Replace Loop with Pipeline（231）
fn total_amount(statement_data: &StatementData) -> u32 {
    statement_data
        .performances
        .iter()
        .map(|p| p.amount)
        .sum::<u32>()
}
// 2.7.2 Replace Loop with Pipeline（231）
fn total_volume_credits(statement_data: &StatementData) -> u32 {
    statement_data
        .performances
        .iter()
        .map(|p| p.total_credits)
        .sum::<u32>()
}
