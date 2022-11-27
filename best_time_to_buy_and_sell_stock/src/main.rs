/*You are given an array prices where prices[i] is the price of a given stock on the ith day.

You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.

Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0*/

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut min_price = std::i32::MAX;
    for n in prices {
        profit = profit.max(n - min_price);
        min_price = min_price.min(n);
    }
    profit
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let result = max_profit(prices);

    println!("Max profit is: {}", result);
}
