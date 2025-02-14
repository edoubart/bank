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

fn print_account(account: Account) -> Account {
    println!("{:#?}", account);
    account
}

fn main() {
    let mut account = Account::new(1, String::from("me"));

    account = print_account(account);

    account = print_account(account);

    println!("{:#?}", account);
}
