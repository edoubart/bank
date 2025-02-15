/***********
 * Structs *
 ***********/

// Attributes
#[derive(Debug)]
struct Account {
    id: u32,
    holder: String,
    balance: i32,
}

// Inherent Implementations
impl Account {
    // Associated Functions
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

// Attributes
#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

// Inherent Implementations
impl Bank {
    // Associated Functions
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}

fn main() {
    let mut bank = Bank::new();

    let account = Account::new(1, String::from("me"));

    bank.add_account(account);

    println!("{:#?}", bank);
}
