use std::{collections::HashMap, hash::Hash};

// We're going to use price for our hash map so we will use Price struct instead of using float64 because it might get us inconsistent values.
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
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

#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
struct Order {
    amount: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    fn new(bid_or_as: BidOrAsk, amount: f64) -> Order {
        Order {
            bid_or_ask: bid_or_as,
            amount: amount,
        }
    }
}

#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    fn new(price: Price) -> Limit {
        Limit {
            price: price,
            orders: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

#[derive(Debug)]
struct Orderbook {
    market: String,
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl Orderbook {
    fn new(market: String) -> Orderbook {
        Orderbook {
            market: market,
            bids: HashMap::new(),
            asks: HashMap::new(),
        }
    }

    fn add_order(&mut self, price: f64, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::Bid => {
                let price = Price::new(price);
                match self.bids.get_mut(&price) {
                    Some(limit) => {
                        println!("order added to limit order");
                        limit.add_order(order);
                    }
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                        println!("orderbook created with limit order");
                    }
                }
            }
            BidOrAsk::Ask => println!(""),
        }
    }
}

fn main() {
    let mut market = String::from("BTC/USDT");
    let mut orderbook = Orderbook::new(market);
    let order1 = Order::new(BidOrAsk::Bid, 5.5);
    let order2 = Order::new(BidOrAsk::Bid, 7.0);
    let order3 = Order::new(BidOrAsk::Bid, 17.0);
    orderbook.add_order(20000.0, order2);
    orderbook.add_order(20000.0, order3);
    orderbook.add_order(10000.0, order1);
    println!("{:?}", orderbook)
}
