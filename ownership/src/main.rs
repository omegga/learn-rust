mod copy;
mod examples;
mod moves;
mod ownership_rules;

fn main() {
    ownership_rules::rules();
    copy::integer_copy();
    copy::deep_copy();
    moves::ownership_vs_copy();
    moves::returning_values();
    examples::mutate_a_vector();
}
