const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;    // 지우고나면 variable does not need to be mutable 에러가 발생
    println!("{} missiles left", missiles);
    
    // READY_AMOUNT = 1; // cannot assign to this expression

    let ext: i32;

}