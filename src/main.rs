use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
struct Account {
    name: String,
    balance: f64,
}

impl Account {
    fn check(&mut self) {
        print!("Enter current balance for {}: ", self.name);
        let mut input = String::new();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let current_balance: f64 = input.trim().parse().unwrap();

        if current_balance == self.balance {
            println!("Account balance unchanged.");
        } else {
            print!(
                "Account balance changed by {}, can you account for the difference? ",
                current_balance - self.balance
            );
            std::io::stdout().flush().unwrap();
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "y" {
                self.balance = current_balance;
                println!("Thank you!");
            } else {
                panic!("Could not sync account.");
            }
        }
    }
}

fn load_accounts() -> Vec<Account> {
    let filename = "./accounts.txt";
    let mut data = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();

    serde_json::from_str(&data).unwrap()
}

fn save_accounts(accounts: &mut [Account]) {
    let filename = "./accounts.txt";
    let data = serde_json::to_string(accounts).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename)
        .unwrap();
    write!(file, "{}", &data).unwrap();
}

fn main() {
    let mut accounts = load_accounts();
    for account in accounts.iter_mut() {
        account.check();
    }
    save_accounts(&mut accounts);
}
