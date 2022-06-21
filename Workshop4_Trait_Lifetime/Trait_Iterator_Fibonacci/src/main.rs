// Bài tập cho trait
// Đề bài : Implement trait Iterator (của thư viện Rust) cho kiểu dữ liệu Struct Fibonnacci

mod fibonacci_iterator;
use fibonacci_iterator::{Fibonacci, fibonacci};

fn main() {
    println!("Kết quả:");
    for i in fibonacci().take(40) {
        println!("> {}", i);
    }
}



