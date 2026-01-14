mod variables;
mod basic_types;

fn main() {
    variables::binding_and_mutability::run();
    variables::scope::run();
    variables::shadowing::run();
    variables::unused_variables::run();
    variables::destructuring::run();
    variables::destructuring_assignments::run();
    
    basic_types::char_bool::run();
    basic_types::functions::run();
    basic_types::numbers::run();
    basic_types::statements_and_expressions::run();
}
