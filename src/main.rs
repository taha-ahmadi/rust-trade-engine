enum BidOrAsk {
    Bid,
    Ask,
}

struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

// We're going to use price for our hash map so we will use Price struct instead of using float64 because it might get us inconsistent values.
#[derive(Debug)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            integral: integral,
            fractional: fractional,
            scalar: scalar,
        }
    }
}

struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Order {
    fn new(bid_or_as: BidOrAsk, size: f64) -> Order {
        Order {
            bid_or_ask: bid_or_as,
            size: size,
        }
    }
}

fn main() {
    let price = Price::new(10.5);
    println!("{:?}", price)
}
