use spellbook::Lunch;

#[expect(clippy::expect_used, reason = "example")]
fn main() {
    buy_lunch_with_money(None).expect_err("should be no free lunch");
}

fn buy_lunch_with_money(
    _: Option<usize>,
) -> Result<Lunch, &'static str> {
    Err("Nice try")
}
