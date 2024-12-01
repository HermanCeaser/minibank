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

    main_account.deposit(10000.0);

    other_account.withdraw(5000.0);
    let balance1 = main_account.balance();
    let balance2 = other_account.balance();

    println!("The current balance for {}'s account ACC No: {} is {}", main_account.holder_name, main_account.account_number, balance1);
    println!("The current balance for {}'s account ACC No: {} is {}", other_account.holder_name, other_account.account_number, balance2);

}

trait Account {
    fn deposit(&mut self, amount:f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&mut self) -> f64;
}

struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount:f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
    }

    fn balance(&mut self) -> f64 {
        return self.balance
    }
}