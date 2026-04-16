mod smart_pointers;

use smart_pointers::{trait_in_box, use_box, use_rc};

fn main() {
    use_box();
    use_rc();
    trait_in_box();
}
