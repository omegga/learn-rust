// A structure is a custom data type that lets you package together
// multiple related values.

mod methods;
mod types_of_structs;
mod update_syntax;
use methods::{calculate_transport_fees, create_international_package};
use types_of_structs::{classic_c_structs, tuple_structs, unit_structs};
use update_syntax::your_order;

fn main() {
    classic_c_structs();
    tuple_structs();
    unit_structs();
    your_order();
    calculate_transport_fees();
    create_international_package();
}
