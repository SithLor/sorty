// loop
// loop 1: repeat 3 time 
//    

pub fn _max_profit(prices: Vec<i32>) -> i32 {
    let mut res = 0;
    let n = prices.len();
    
    
    let conut_of_day = prices.len()/2;
    for i in 0..conut_of_day {
        let mut min = prices[i];
        for j in i + 1..conut_of_day {
            if prices[j] < min {
                min = prices[j];
            }
            if prices[j] - min > res {
                res = prices[j] - min;
            }
        }
    }
    res
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut res = 0;
    let n = prices.len();

    // Explore all possible ways to buy and sell stock
    // length - 1 
    for i in 0..n - 1 {

        for j in i + 1..n {
            if prices[j] - prices[i] > res {
                res = prices[j] - prices[i];
            }
        }
    }
    res
}

fn main() {
    let prices = vec![7, 6, 4, 3, 1];
    let result = max_profit(prices);
    println!("The result is: {}", result);
    println!("Hello, world!");
}