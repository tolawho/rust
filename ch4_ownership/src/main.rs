fn main() {
    // ownership
    let s = String::from("Hello");
    let s2 = s.clone(); // clone
    println!("{:?}", _cal_lenght(s));
    println!("s2 -> {}", s2);

    // reference
    let mut x = String::from("Hello");
    let len = _cal_lenght_with_ref(&mut x);
    println!("{} length is {}", x, len);
    println!("{:?}", x.as_bytes().to_vec());

    //slice type
    let s = String::from("Hello World");
    let hello = &s[0..5];
    let world = &s[6..];
    println!("{}", hello);
    println!("{}", world);
}

fn _cal_lenght(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn _cal_lenght_with_ref(x: &mut String) -> usize {
    x.push_str(" world!");
    x.len()
}
