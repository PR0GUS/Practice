mod variables;
mod basic_types;
mod ownership_and_borrowing;
mod flow_control;
mod pattern_match;
mod method;

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

    ownership_and_borrowing::ownership::run();
    ownership_and_borrowing::reference_and_borrowing::run();

    flow_control::for_::run();
    flow_control::continue_and_break::run();
    flow_control::if_else::run();
    flow_control::loop_::run();
    flow_control::while_::run();
    
    pattern_match::patterns::run();
    pattern_match::match_and_if_let::run();

    method::method_and_associated_functions::run();
}


