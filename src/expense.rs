use std::fmt;

pub struct Expense {
	pub name: String,
	pub price: u64,
	pub status: bool,
}

impl fmt::Display for Expense {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "${:>5} {}\t{}", self.price, self.name, if self.status { "OK" } else { "..." } )
	}
}

impl Expense {
	pub fn from_line(line: &str) -> Expense {
		let line = line.to_owned();

		let splited: Vec<&str> = line.split("-").collect();

		let price:u64 = splited[0].parse().unwrap();
		let name = splited[1].to_owned();
		let status = splited[2] == "1";

		Expense {
			name,
			price,
			status,
		}
	}

	pub fn to_line(&self) -> String {
		format!("{}-{}-{}", self.price, self.name, if self.status { "1" } else { "0" })
	}
}