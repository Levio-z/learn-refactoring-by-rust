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
// The first step is to apply Replace Type Code with Subclasses (362) to
// introduce subclasses and deprecate the type code.
fn enrich_performance(perf: &Performance, plays: &HashMap<String, Play>) -> PerformanceData {
    let mut result = PerformanceData {
        audience: perf.audience,
        ..Default::default()
    };
    let calculator = create_performance_calculator(perf, play_for(perf, plays));
    result.amount = calculator.get_amount();
    result.total_credits = calculator.get_volume_credits();
    result.play = calculator.get_play().clone();
    result
}
// The factory function determines which subclass instance to return.
fn create_performance_calculator(perf: &Performance, play: Play) -> Box<dyn PerformanceCalculator> {
    match play.kind.as_str() {
        "tragedy" => Box::new(TragedyCalculator {
            base: PerformanceCalculatorBase {
                performance: perf.clone(),
                play,
            },
        }),
        "comedy" => Box::new(ComedyCalculator {
            base: PerformanceCalculatorBase {
                performance: perf.clone(),
                play,
            },
        }),
        _ => panic!("unknown type: {}", play.kind),
    }
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
pub(crate) struct PerformanceCalculatorBase {
    pub performance: Performance,
    pub play: Play,
}
trait PerformanceCalculator {
    fn audience(&self) -> i32;

    fn get_amount(&self) -> u32;

    fn get_play(&self) -> &Play;

    fn get_volume_credits(&self) -> u32 {
        self.base_volume_credits()
    }

    fn base_volume_credits(&self) -> u32 {
        (self.audience() - 30).max(0) as u32
    }
}

struct TragedyCalculator {
    base: PerformanceCalculatorBase,
}
impl PerformanceCalculator for TragedyCalculator {
    fn get_amount(&self) -> u32 {
        let mut result = 40000;
        if self.audience() > 30 {
            result += 1000 * (self.audience() - 30) as u32;
        }
        result
    }

    fn audience(&self) -> i32 {
        self.base.performance.audience as i32
    }
    fn get_play(&self) -> &Play {
        &self.base.play
    }
}

struct ComedyCalculator {
    base: PerformanceCalculatorBase,
}
impl PerformanceCalculator for ComedyCalculator {
    fn get_amount(&self) -> u32 {
        let mut result = 30000;
        if self.audience() > 20 {
            result += 10000 + 500 * (self.audience() - 20) as u32;
        }
        result += 300 * self.audience() as u32;
        result
    }
    fn get_volume_credits(&self) -> u32 {
        self.base_volume_credits() + self.audience() as u32 / 5
    }
    fn audience(&self) -> i32 {
        self.base.performance.audience as i32
    }
    fn get_play(&self) -> &Play {
        &self.base.play
    }
}
