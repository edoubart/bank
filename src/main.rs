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
}

fn print_account(account: Account) {
    println!("{:#?}", account);
}

fn main() {
    let bank = Bank::new();
    let account = Account::new(1, String::from("me"));

    print_account(account);
}
