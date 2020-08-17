// Lifetimes enforce a piece of memory is still valid for a reference

mod examples;
mod reminder;

fn main() {
    reminder::example_1();
    // reminder::example_2();
    // examples::get_int_ref();
    examples::get_int_ref_2();
    examples::get_int_ref_3(&777);
    examples::get_int_ref_4(&777);
    examples::get_int_ref_5(&777, &777);
    // examples::get_int_ref_6(&777, &777);
    // examples::get_int_ref_7(&777, &777);
}
