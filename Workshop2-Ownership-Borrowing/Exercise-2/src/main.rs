//Exercise 2
// Mục đích: giải quyết vấn đề ownership và borrowing ko dùng clone()
// Các bạn có thể sửa thêm logic để đúng với mục đichs bài này là liệt kê các số nguyên tố 
fn main() {
    let mut count: u32 = 1; 
    let range = 10;
    let mut num: u64 = 1;
    let mut primes: Vec<u64> = Vec::new();
    primes.push(2);

    while count < range {
        num += 2;
        if vector_is_prime(num, &primes) {            
            primes.push(num);
        }
        count += 1;
    }
    println!("Các số nguyên tố trong phạm vi {} là: {:?}", range, primes);
}

fn vector_is_prime(num: u64, p: &Vec<u64>) -> bool {
    for i in p {
        if num > *i && num % i == 0 {
            return false;
        }
    }

    true
}