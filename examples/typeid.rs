use std::any::{Any as _, TypeId};

fn main() {
    let hidden = String::from("what could it be?");
    println!("String: {:?}", TypeId::of::<String>());
    println!("hidden: {:?}", hidden.type_id());
    // String: TypeId(0x54661232f4503ad5453a68e57906db78)
    // hidden: TypeId(0x54661232f4503ad5453a68e57906db78)
}
