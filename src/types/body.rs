use super::header::Header;

pub struct Body {
    transactions: Vec<Transaction>,
    uncles: Vec<Header>,
}

struct Transaction {}
