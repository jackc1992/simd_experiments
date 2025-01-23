pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_rounding() {
        let len = 4096;
        let len = (len + 4095) & !4095;

        println!("len: {}, diff: {}", len, 0);
        assert!(len % 4096 == 0);
        assert!(len == 4096);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
