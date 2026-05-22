#![expect(clippy::unnecessary_wraps, reason = "example")]
#[expect(clippy::expect_used, reason = "demonstrating 'expect'")]
fn main() {
    let possible_cake = try_mixing_batter()
        .and_then(try_baking_cake)
        .and_then(try_chocolate_icing);
    possible_cake.expect("You have failed me for the last time, Chef");
}

fn try_mixing_batter() -> Result<(), ()> {
    Ok(())
}

fn try_baking_cake(_batter: ()) -> Result<(), ()> {
    Ok(())
}

fn try_chocolate_icing(_cake: ()) -> Result<(), ()> {
    Ok(())
}
