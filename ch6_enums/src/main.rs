#[derive(Debug)]
struct User {
    name: String,
    sex: Sex,
}

#[derive(Debug)]
enum Sex {
    Male,
    Female,
}

impl User {
    fn gioi_tinh(&self) -> String {
        match self.sex {
            Sex::Male => "Nam".to_string(),
            Sex::Female => "Nu".to_string(),
        }
    }
}

fn main() {
    let member = new_member("tolawho".to_string(), Sex::Male);
    println!("{:#?}", member);
    println!("{}", member.name);
    println!("Gioi tinh {}", member.gioi_tinh());

    let member = new_member("tolawho2".to_string(), Sex::Female);
    println!("{:#?}", member);
    println!("Gioi tinh {}", member.gioi_tinh());

    // use enum option<T>
    let some_number: Option<i32> = Some(4);
    println!("{:?}", some_number);
    let some_string: Option<&str> = Some("Day la String");
    println!("{:?}", some_string);
    let none_number: Option<i32> = None;
    println!("{:?}", none_number);

    let s = String::from("thanh");
    println!("{}", s);

    let x = 5;
    let y = None;
    let sum = x + y.unwrap_or(1);
    println!("Sum is {}", sum);

    // match option
    let four = Some(4);
    let five = plus_one(four);
    println!("{:#?}", five);

    let none: Option<i32> = plus_one(None);
    println!("{:#?}", none);
}

fn new_member(name: String, sex: Sex) -> User {
    User { name, sex }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    x.map(|x| x + 1)
}
