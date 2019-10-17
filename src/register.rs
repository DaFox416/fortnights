use std::io::prelude::*;
use std::fs::OpenOptions;

use read_input::prelude::*;

use crate::fortnight::Fortnight;
use crate::expense::Expense;

pub struct Register {
	filename: String,
	fortnights: Vec<Fortnight>,
	i: usize,
}

impl Register {
	pub fn load_file(filename: String) -> Register {
		let mut temp = String::from("data/");
		temp.push_str(filename.as_ref());
		let filename: &str = temp.as_ref();
		let mut fortnights = Vec::new();

		println!("Loading data...");
		let mut file = OpenOptions::new().read(true).write(true).open(filename).unwrap();
		let mut content = String::new();
		file.read_to_string(&mut content).unwrap();

		if content == "" {
			let init = String::from("0:");
			file.write_all(init.as_bytes()).unwrap();
			content = String::from(init.as_ref() as &str);
		}

		let mut i=0;
		for line in content.lines() {
			i+=1;
			fortnights.push(Fortnight::from_line(line));
		}
		i-=1;

		println!("Success!");

		Register {
			filename: String::from(filename),
			fortnights,
			i,
		}
	}
}

impl Register {
	pub fn add_cmd(&mut self, args: Vec<String>) {
		if args.is_empty() {
			println!("Error: Missing arguments!");
		} else {
			let mut sub_args: Vec<String> = Vec::new();
			for (i, arg) in args.iter().enumerate() {
				if i!=0 {
					sub_args.push(arg.clone());
				}
			}
			match args[0].as_ref() {
				"-e" => self.add_expense(),
				"-f" => self.add_fortnight(),
				_ => println!("Error: unexpected argument '{}'", args[0]),
			}
		}
	}

	pub fn edit_cmd(&mut self, args: Vec<String>) {
		if args.is_empty() {
			println!("Error: Missing arguments!");
		} else {
			let mut sub_args: Vec<String> = Vec::new();
			for (i, arg) in args.iter().enumerate() {
				if i!=0 {
					sub_args.push(arg.clone());
				}
			}
			match args[0].as_ref() {
				"-e" => self.edit_expense(),
				"-f" => self.edit_fortnight(),
				_ => println!("Error: unexpected argument '{}'", args[0]),
			}
		}
	}

	pub fn increase_cmd(&mut self, args: Vec<String>) {
		if args.is_empty() {
			let value: u64 = input()
				.repeat_msg("Enter increase value: ")
				.get();
			self.fortnights[self.i].budget += value;
			println!("New budget: ${}", self.fortnights[self.i].budget);
		} else {
			println!("Error: Too much arguments!");
		}
	}

	pub fn list_cmd(&self, args: Vec<String>) {
		if args.is_empty() {
			println!("{}", self.fortnights[self.i]);
		} else {
			match args[0].as_ref() {
				"-a" => {
					println!();
					for (i, fort) in self.fortnights.iter().enumerate() {
						println!("{}", fort);
						if i<self.fortnights.len()-1 {
							println!("-------------------");
						}
					}
					println!();
				},
				_ => println!("{} is not a valid argument!", args[0]),
			}
		}
	}
 
	pub fn remove_cmd(&mut self, args: Vec<String>) {
		if args.is_empty() {
			println!("Error: Missing arguments!");
		} else {
			let mut sub_args: Vec<String> = Vec::new();
			for (i, arg) in args.iter().enumerate() {
				if i!=0 {
					sub_args.push(arg.clone());
				}
			}
			match args[0].as_ref() {
				"-e" => self.remove_expense(),
				"-f" => self.remove_fortnight(),
				_ => println!("Error: unexpected argument '{}'", args[0]),
			}
		}
	}

	pub fn set_cmd(&mut self, args: Vec<String>) {
		self.list_cmd(Vec::new());
		if args.is_empty() {
			println!("Error: Missing arguments!");
		} else {
			let i_exp: usize;
			let arg: &str = args[0].as_ref();
			match arg {
				"-so" | "-uo" => {
					i_exp = input::<usize>()
						.repeat_msg("Enter index: ")
						.inside(0..self.fortnights[self.i].expenses.len())
						.default(self.fortnights[self.i].expenses.len()-1)
						.get();
				},
				"-sa" | "-ua" => {
					for exp in self.fortnights[self.i].expenses.iter_mut() {
						exp.status = arg == "-sa";
					}
					return;
				},
				_ => {
					println!("Error: Unexpected argument '{}'!", args[0]);
					return;
				},
			}
			self.fortnights[self.i].expenses[i_exp].status = arg=="-so";
		}
	}

	pub fn step_cmd(&mut self, args: Vec<String>) {
		if args.is_empty() {
			if self.i == self.fortnights.len()-1 {
				println!("Can´t step forward!");
			} else {
				self.i+=1;
			}
		} else {
			match args[0].as_ref() {
				"-b" => {
					if self.i == 0 {
						println!("Can't step back!");
					} else {
						self.i-=1;
					}
				},
				"--newest" => {
					self.i = self.fortnights.len()-1;
				},
				"--first" => {
					self.i = 0;
				}
				_ => println!("Error: unexpected argument '{}'", args[0]),
			}
		}
	}

	pub fn add_expense(&mut self) {
		let mut name = String::new();
		while name.len()<5 || name.len()>11 {
			name = input()
				.msg("Name: ")
				.get();
		}
		let price = input::<u64>()
			.repeat_msg("Price: ")
			.inside(1..=self.fortnights[self.i].get_remaining(true))
			.get();
		let set : String = input()
			.msg("Is set? Y/N: ")
			.get();
		let c = if set == "Y" || set == "y" { '1' } else { '0' };
		let exp = Expense::from_line(format!("{}-{}-{}", price, name, c).as_ref());
		println!("Adding expense: {}", exp);
		self.fortnights[self.i].expenses.push(exp);
	}

	pub fn add_fortnight(&mut self) {
		let mut budget = input::<u64>()
			.repeat_msg("Budget: ")
			.get();
		
		let remainder = self.fortnights[self.i].get_remaining(true);
		println!("Adding last remainder: ${}", remainder);
		budget += remainder;
		self.fortnights[self.i].budget -= remainder;

		let fort = Fortnight::from_line(format!("{}:", budget).as_ref());
		println!("Adding fortnight:\n{}", fort);
		self.fortnights.push(fort);
		self.i+=1;
	}

	pub fn edit_expense(&mut self) {
		self.list_cmd(Vec::new());
		let i_exp = input::<usize>()
			.repeat_msg("Enter index (Last by default): ")
			.default(self.fortnights[self.i].expenses.len()-1)
			.get();

		println!("Enter new expense data. (Press ENTER for no changes)");
		
		let name: String = input()
			.msg("New name: ")
			.default(self.fortnights[self.i].expenses[i_exp].name.clone())
			.get();
		let price = input::<u64>()
			.repeat_msg("New price: ")
			.inside(1..=(self.fortnights[self.i].get_remaining(true) + self.fortnights[self.i].expenses[i_exp].price))
			.default(self.fortnights[self.i].expenses[i_exp].price)
			.get();
		let status: String = input()
			.msg("Set to OK? Y/N: ")
			.default(if self.fortnights[self.i].expenses[i_exp].status { String::from("Y") } else { String::from("N") })
			.get();
		
		self.fortnights[self.i].expenses[i_exp].status = status == "Y";
		self.fortnights[self.i].expenses[i_exp].name = name.clone();
		self.fortnights[self.i].expenses[i_exp].price = price;
	}

	pub fn edit_fortnight(&mut self) {
		let i_fort = input::<usize>()
			.repeat_msg("Enter index (Current by default): ")
			.inside(0..self.fortnights.len())
			.default(self.i)
			.get();
		
		let budget = input::<u64>()
			.repeat_msg("New budget: ")
			.get();
		
		self.fortnights[i_fort].budget = budget;
	}

	pub fn remove_expense(&mut self) {
		if self.fortnights[self.i].expenses.is_empty() { println!("No expense to remove!"); return; }
		self.list_cmd(Vec::new());

		let i_exp = input::<usize>()
			.repeat_msg("Expense index: (Last by default)")
			.inside(0..self.fortnights[self.i].expenses.len())
			.default(self.fortnights[self.i].expenses.len()-1)
			.get();

		println!("Removing expense: {}", self.fortnights[self.i].expenses[i_exp]);
		self.fortnights[self.i].expenses.remove(i_exp);
	}

	pub fn remove_fortnight(&mut self) {
		if self.fortnights[0].to_line() == "0:" && self.fortnights.len()==1 { println!("No fortnight to remove!"); return; }

		let i_fort = input::<usize>()
			.repeat_msg("Enter index: (Current by default)")
			.inside(1..self.fortnights.len())
			.default(self.i)
			.get();

		println!("Removing fortnight: {}", self.fortnights[i_fort]);
		self.fortnights.remove(i_fort);
		if self.fortnights.is_empty() {
			self.fortnights.push(Fortnight::from_line("0:"));
		}
		self.i = self.fortnights.len()-1;
	}
}

impl Register {
	pub fn save_in_file(&self) {
		let mut file = OpenOptions::new().write(true).truncate(true).open(self.filename.clone()).unwrap();

		let mut content = String::new();
		for (i, fort) in self.fortnights.iter().enumerate() {
			content.push_str(fort.to_line().as_ref());
			if i<self.fortnights.len()-1 {
				content.push('\n');
			}
		}
		file.write_all(content.as_bytes()).unwrap();
	}

	pub fn get_i(&self) -> usize {
		self.i
	}

	pub fn get_current(&self) -> &Fortnight {
		&self.fortnights[self.i]
	}
}