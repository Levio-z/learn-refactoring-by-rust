use std::collections::HashMap;

use super::{Invoice, Performance, Play};

#[derive(Debug, Clone, Default)]
pub(crate) struct PerformanceData {
    pub play: Play,
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

fn play_for(perf: &Performance, plays: &HashMap<String, Play>) -> Play {
    plays.get(&perf.play_id).expect("Play not found").clone()
}

// 1.Create a performance calculator
// 2.Initialize the calculator with play data
// 3.Move amount calculation into the calculator
// 4.Refactor the original function into a delegating function so that it
// directly calls the new function.
// 5.Compile, test, and commit to ensure the code works as usual after being
// moved to its new home. Then I applied Inline Function (115), so that the call
// sites invoke the new function directly (followed by compile, test, and
// commit).
// 5.1 Move volume credits calculation into the calculator
// 5.2 Move volume amount calculation into the calculator
fn enrich_performance(perf: &Performance, plays: &HashMap<String, Play>) -> PerformanceData {
    let calculator = PerformanceCalculator {
        play: play_for(perf, plays),
        performance: perf.clone(),
    };
    let mut result = PerformanceData {
        audience: perf.audience,
        ..Default::default()
    };
    result.play = calculator.play.clone();
    result.amount = calculator.get_amount();
    result.total_credits = calculator.get_volume_credits();
    result
}

fn total_amount(statement_data: &StatementData) -> u32 {
    statement_data
        .performances
        .iter()
        .map(|p| p.amount)
        .sum::<u32>()
}

fn total_volume_credits(statement_data: &StatementData) -> u32 {
    statement_data
        .performances
        .iter()
        .map(|p| p.total_credits)
        .sum::<u32>()
}

#[derive(Debug, Clone, Default)]
pub(crate) struct PerformanceCalculator {
    pub play: Play,
    pub performance: Performance,
}
impl PerformanceCalculator {
    pub fn get_amount(&self) -> u32 {
        let mut result;
        match self.play.kind.as_str() {
            "tragedy" => {
                result = 40000;
                if self.performance.audience > 30 {
                    result += 1000 * (self.performance.audience - 30);
                }
            }
            "comedy" => {
                result = 30000;
                if self.performance.audience > 20 {
                    result += 10000 + 500 * (self.performance.audience - 20);
                }
                result += 300 * self.performance.audience;
            }
            _ => panic!("unknown type: {}", self.play.kind),
        }
        result
    }

    fn get_volume_credits(&self) -> u32 {
        let mut result = 0;
        result += std::cmp::max(self.performance.audience as i32 - 30, 0) as u32;
        if self.play.kind == "comedy" {
            result += self.performance.audience / 5;
        }
        result
    }
}
