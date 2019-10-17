use std::fmt;

use crate::expense::Expense;

pub struct Fortnight {
    pub budget: u64,
	pub expenses: Vec<Expense>,
}

impl fmt::Display for Fortnight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Budget: ${}", self.budget)?;
        for (i, ex) in self.expenses.iter().enumerate() {
            writeln!(f, "{}.- {}", i, ex )?;
        }
        writeln!(f, "${:>5} Remaining", self.get_remaining(false))?;
        write!(f, "${:>5} Available", self.get_remaining(true))
    }
}

impl Fortnight {
    pub fn from_line(line: &str) -> Fortnight {
        let line = line.to_owned();
        let mut expenses = Vec::new();

        let first_split: Vec<&str> = line.split(":").collect();
        let budget: u64 = first_split[0].parse().unwrap();

        if first_split.len() > 1 && first_split[1] != "" {
            let expnss = first_split[1].to_owned();
            let second_split: Vec<&str> = expnss.split("|").collect();

            for ex in second_split.iter() {
                expenses.push(Expense::from_line(ex));
            }
        }

        Fortnight {
            budget,
            expenses,
        }
    }

    pub fn to_line(&self) -> String {
        let mut line = String::new();
        line.push_str(format!("{}:", self.budget).as_ref());
        for (i, exp) in self.expenses.iter().enumerate() {
            line.push_str(exp.to_line().as_ref());
            if i<self.expenses.len()-1 {
                line.push('|');
            }
        }
        line
    }

    pub fn get_remaining(&self, count_all:bool) -> u64 {
        let mut sum = 0;
        for ex in self.expenses.iter() {
            if ex.status || count_all {
                sum += ex.price;
            }
        }
        if sum > self.budget {
            0
        } else {
            self.budget-sum
        }
    }
}