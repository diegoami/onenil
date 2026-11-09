fn main() {
    let c = onenil_campaign::Campaign::initial().unwrap();
    println!("next_fixture: {:?}", c.next_fixture);
    println!("week_index: {}", c.week_index);
    println!("cash: {}", c.cash);
}
