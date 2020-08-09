mod heap_strings;
mod string_literals;
mod string_or_slice;

fn main() {
    string_literals::create_a_string_literal();
    heap_strings::the_string_type();
    heap_strings::string_mutation();
    string_or_slice::guess();
}
