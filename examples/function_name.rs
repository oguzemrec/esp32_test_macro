use ::esp32_test_macro::esp32_test;


#[esp32_test]
fn my_function() {
    println!("Inside my_function");
}

fn main() {
    my_function();
}