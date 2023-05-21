pub fn q1() -> usize {
    let key = "ckczppom";
    let mut num: usize = 0;
    loop {
        let str = format!("{}{}", key, num);
        let r = md5::compute(&str);
        let as_str = format!("{:?}", r);
        if &as_str[..5] == "00000" {
            return num;
        }
        num += 1
    }
}

pub fn q2() -> usize {
    let key = "ckczppom";
    let mut num: usize = 0;
    loop {
        let str = format!("{}{}", key, num);
        let r = md5::compute(&str);
        let as_str = format!("{:?}", r);
        if &as_str[..6] == "000000" {
            return num;
        }
        num += 1
    }
}
