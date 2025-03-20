#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}


#[ic_cdk::query]
fn calculate(a: i32, b: i32, operator: String)  -> i32 {
    let result = match operator.as_str() {
        "+" => Some(a+b),
        "-" => Some(a-b),
        "*" => Some(a*b),
        "/" => b !== 0 {Some(a/b)} else {None},
        _ => None
    }

    match {
        Some(val) => format!("Result {}", val),
        None => "Invalid operator or division by zero".to_string();
    }

}