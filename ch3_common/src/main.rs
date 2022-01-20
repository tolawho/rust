fn main() {
    println!("Rust common programming concepts");
    variables_and_mutability();

    datatypes();

    // function call
    _fn_with_params(5);

    // control flow
    _fn_control_flow();
}

fn variables_and_mutability() {
    let mut x = 5;

    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    // constants
    _constants();
    // shadowing
    _shadowing();
}

fn _constants() {
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("constants value is {}", THREE_HOURS_IN_SECONDS);
}

fn _shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {}", x);
    }
    println!("The value of x is {}", x);
}

fn datatypes() {
    let x = 2.01; // f64
    println!("value of float x is {}", x);

    let y: f32 = 3.01; // f32
    println!("value of float y is {}", y);

    // byte
    println!("byte {}", b'A');

    // numeric operations
    // addition
    let sum = 5 + 10;
    println!("value of sum is {}", sum);

    // subtraction
    let subtraction = 5 - 10;
    println!("value of subtraction is {}", subtraction);

    // mutiplication
    let multiplication = 5 * 10;
    println!("value of multiplication is {}", multiplication);

    // division
    let division = 5 / 10;
    println!("value of division is {}", division);

    // remainder
    let remainder = 13 % 10;
    println!("value of remainder is {}", remainder);

    // boolean
    let x = true;
    let y: bool = false;
    println!("value of x is {}", x);
    println!("value of y is {}", y);

    // character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("value of c is {}", c);
    println!("value of z is {}", z);
    println!("value of heart_eyed_cat is {}", heart_eyed_cat);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("value of tup is {:?}", tup);
    let (x, y, z) = tup;
    println!("(x,y,x) = ({},{},{})", x, y, z);
    println!("value of tup.0 is {}", tup.0);
    println!("value of tup.1 is {}", tup.1);
    println!("value of tup.2 is {}", tup.2);

    // array define
    let a = [1, 2, 3, 4, 5];
    println!("array a = {:?}", a);
    let a1 = [5; 10];
    println!("array a1 = {:?}", a1);

    // array accessing
    println!("value of a[0] is {}", a[0]);
}

fn _fn_with_params(x: i32) {
    // statement
    let c = 10;
    println!("value of c is {}", c);

    println!("fn value of x params is {}", x);

    // expression
    let y = { x + 1 };
    println!("value of y is {}", _fn_with_return_value(y));
}

fn _fn_with_return_value(x: i32) -> i32 {
    x
}

// control flow

fn _fn_control_flow() {
    // if expression
    let number = 3;
    if number == 3 {
        println!("number was three");
    }
    if number < 5 {
        println!("number less than 5");
    }

    if number % 4 == 0 {
        println!("number {} divisible by 4", number);
    } else if number % 3 == 0 {
        println!("number {} divisible by 3", number);
    } else if number % 2 == 0 {
        println!("number {} divisible by 2", number);
    } else {
        println!("number not divisible by 2,3,4");
    }

    // repetition with loops
    let mut count = 0;
    'hello_loop: loop {
        println!("count: {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'hello_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }

    // condition loop with while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Liftoff!!!");

    // loop with for
    let a = [5; 20];
    for item in a {
        println!("value is {}", item);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    for number in 1..4 {
        println!("{}!", number);
    }
}
