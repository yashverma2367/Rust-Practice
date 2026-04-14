mod scope;
fn main() {
    scope::tranfer_ownership();

    scope::move_ownership();

    scope::clone_value();

    scope::copy_types();
    println!("Memory Management")
}
