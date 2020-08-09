mod any_error;
mod propagate_errors;
mod result;

fn main() {
    result::example();
    propagate_errors::example();
    propagate_errors::example_2().expect("error!");
    any_error::example().expect("error!");
}
