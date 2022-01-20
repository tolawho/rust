#[derive(Debug)]
struct Member {
    username: String,
    email: String,
    age: u64,
    active: bool,
}

#[derive(Debug)]
struct KichThuoc {
    dai: u32,
    rong: u32,
}

fn main() {
    let mut member1: Member = Member {
        username: String::from("tolawho"),
        email: String::from("tolawho@gmail.com"),
        age: 35,
        active: false,
    };

    let username = member1.username;
    println!("name = {}", username);
    println!("email = {}", member1.email);
    println!("age = {}", member1.age);
    println!("is active = {}", member1.active);

    member1.username = "thanh".to_string();

    let member2 = create_new_member(member1.username, String::from("tolawho2@gmail.com"), 25);
    println!("{:#?}", member2);
    let member3 = Member {
        username: "hi".to_string(),
        ..member2
    };
    println!("{:#?}", member3);

    let kt = KichThuoc { dai: 10, rong: 12 };
    println!("Hinh chu nhat => {:#?}", kt);
    println!("Dien tich {}", dien_tich(kt));
}

fn create_new_member(username: String, email: String, age: u64) -> Member {
    Member {
        username,
        email,
        age,
        active: true,
    }
}

fn dien_tich(kt: KichThuoc) -> u32 {
    kt.dai * kt.rong
}
