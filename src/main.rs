fn main() {
    let mut main_account = BankAccount {
        account_number: String::from("1034101544435"),
        holder_name: String::from("John Doe"),
        balance: 0.0
    };

    let mut other_account = BankAccount {
        account_number: String::from("1034101644436"),
        holder_name: String::from("Jane Doe"),
        balance: 20000.0
    };

    match main_account.deposit(10000.0) {
        Ok(_) => println!("Successfully Deposited 10000.0 into  {}'s account", main_account.holder_name),
        Err(e) => println!("Failed to deposit into {}'s account: {}", main_account.holder_name, e),
    }

    match main_account.withdraw(15000.0) {
        Ok(_) => println!("Successfully Withdrew 15000.0 from  {}'s account", main_account.holder_name),
        Err(e) => println!("Failed to withdraw 15000 from  {}'s account: {}", main_account.holder_name, e),
    }

    match other_account.withdraw(5000.0) {
        Ok(_) => println!("Successfully withdrew 5000.0 into  {}'s account", other_account.holder_name),
        Err(e) => println!("Error withdrawing funds: {}", e),
    }
    
    let balance1 = main_account.balance();
    let balance2 = other_account.balance();

    println!("The current balance for {}'s account ACC No: {} is {}", main_account.holder_name, main_account.account_number, balance1);
    println!("The current balance for {}'s account ACC No: {} is {}", other_account.holder_name, other_account.account_number, balance2);

}

trait Account {
    fn deposit(&mut self, amount:f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&mut self) -> f64;
}

struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount:f64) -> Result<(), String> {
        if amount > 0.0 {
            self.balance += amount;
            Ok(())
        } else {
            Err("Deposit amount must be greater than zero!".to_string())
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Withdrawal amount must be greater than zero.".to_string())
        } else if amount > self.balance {
            Err("Insufficient balance.".to_string())
        } else {
            self.balance -= amount;
            Ok(())
        }
    }

    fn balance(&mut self) -> f64 {
        return self.balance
    }
}