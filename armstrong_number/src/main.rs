pub fn main(num: u32) -> bool {
    let mut digits = Vec::new();
    let mut num2 = num;
    let mut i = 0;

    while num2 > 0 {
        digits.push(num2 % 10);
        num2 /= 10;
        i += 1;
    }

    digits.iter().map(|x| x.pow(i)).sum::<u32>() == num
}
