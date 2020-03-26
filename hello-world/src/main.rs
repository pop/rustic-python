
#[derive(Debug)]
struct SomeError;

#[derive(Debug)]
enum Thing {
    First,
    Second,
    Third,
}

#[derive(Debug)]
struct Pair {
    num_member: u32,
    str_member: String,
}


fn my_function(pair: Pair) -> Result<u32, SomeError> {

    // Parse the str_member as an integer
    let str_as_num_result = pair.str_member.parse::<u32>();

    // Check if the result is OK
    if str_as_num_result.is_ok() {
        // Unwrap the OK result
        let str_as_num = str_as_num_result.unwrap();

        // Add the numbers together
        let result = str_as_num + pair.num_member;

        // Return the OK result
        return Ok(result);
    } else {
        // We got an error, so raise an error!
        return Err(SomeError)
    }
}

fn my_function_match(pair: Pair) -> Result<u32, SomeError> {
    match pair.str_member.parse::<u32>() {
        Ok(str_as_num) => Ok(str_as_num + pair.num_member),
        Err(_oops) => Err(SomeError),
    }
}

fn evaluate_request(req: Thing) -> String {
    match req {
        Thing::First => String::from("You picked the first thing!"),
        Thing::Second => String::from("You have wonderful taste!"),
        Thing::Third => String::from("Risk Taker!!"),
    }
}

fn main() {
    println!("{}", evaluate_request(Thing::Third));
    println!("{:?}", my_function(Pair { num_member: 1, str_member: String::from("2") }));
    println!("{:?}", my_function(Pair { num_member: 2, str_member: String::from("Foo") }));
    println!("{:?}", my_function_match(Pair { num_member: 3, str_member: String::from("2") }));
    println!("{:?}", my_function_match(Pair { num_member: 4, str_member: String::from("Foo") }));
}
