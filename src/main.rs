#[derive(Debug)]
struct Account {
    id: u32,
    holder: String,
    balance: i32,
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

fn main() {
    println!("Hello, world!");
}
