macro_rules! calculate {
    (add $x:expr, $y:expr) => {
        $x + $y
    };
    (subtract $x:expr, $y:expr) => {
        $y - $x
    };
}

#[test]
fn test() {
    let result_add = calculate!(add 3, 5);
    let result_subtract = calculate!(subtract 2, 8);

    println!("Result of addition: {}", result_add);
    println!("Result of subtraction: {}", result_subtract);
}

#[derive(Debug)]
struct Ident {
    value: String,
    start: u32,
    end: u32,
}

macro_rules! ident {
    ($value: expr) => {
        Ident {
            value: $value,
            start: 0,
            end: 0,
        }
    };
}

#[test]
fn test_ident_creation() {
    let ident = ident!(String::from("123"));
    println!("{:?}", ident);
}
