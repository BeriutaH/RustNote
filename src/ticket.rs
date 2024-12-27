use std::sync::{Arc, Mutex};
use std::thread;

pub fn run_ticket_sale() {
    // 初始化票数
    let tickets = Arc::new(Mutex::new(10));
    let mut handles = vec![];

    // 模拟10个用户抢票
    for _ in 0..10 {
        let tickets = Arc::clone(&tickets);
        let handle = thread::spawn(move || {
            let mut num = tickets.lock().unwrap();
            if *num > 0 {
                *num -= 1;
                println!("抢到票了！剩余票数: {}", *num);
            } else {
                println!("票已售罄！");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("抢票结束，剩余票数: {}", *tickets.lock().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_ticket_sale() {
        run_ticket_sale();
        // 由于抢票过程是并发的，无法确定具体的输出内容，
        // 这里只是调用函数，确保没有panic。
    }
}
