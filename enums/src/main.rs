mod associated_value;
mod create;
mod messages;
mod methods;
mod with_struct;

fn main() {
    create::create_enum();
    with_struct::with_a_struct();
    associated_value::ex_1();
    associated_value::ex_2();
    associated_value::ex_3();
    methods::enum_method();
    messages::match_message_call();
}
