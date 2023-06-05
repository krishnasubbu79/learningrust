struct Bankaccount {
    balance: f64,
}

impl Bankaccount {
    // Create new
    fn new(initial_balance: f64) -> Bankaccount {
        Bankaccount {
            balance: initial_balance
        }
    }

    // Add money to account - deposit.
    // &mut self -> refers to the instance of the struct that the method is being called upon.
    // self refers to the instance of the struct - similar to python.
    // & before self -> signifies that the method is taking reference to self instead of taking ownership.
    // its just borrowing self for the duration of the method.

    // mut before self denotes that the reference is mutable.
    // This means that the method can change the fields of the struct.
    // Without mut, the method would not be allowed to change the state of the instance.

    // So, &mut self as a method parameter allows the method to change the fields of the struct instance
    // it's called on, but without taking ownership of the instance.

    fn deposit(&mut self, amount: f64) {
        self.balance = self.balance + amount;
    }

    // Withdraw money - dont allow withdraw if amount passed to withdraw is greater than the available balance.
    fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance = self.balance - amount;
        } else {
            println!("Withdrawal denied - Insufficient Funds")
        }
    }

    // report only balance
    fn get_balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // Initialize
    let mut account = Bankaccount::new(100.0);
    println!("Initial balance: {}", account.get_balance());

    account.deposit(50.0);
    println!("Initial balance: {}", account.get_balance());

    account.withdraw(100.0);
    println!("Initial balance: {}", account.get_balance());
}