use spellbook::Lunch;

fn main() {
    buy_lunch_without_money().expect_err("should be no free lunch");
}

fn buy_lunch_without_money() -> Result<Lunch, &'static str> {
    Err("Nice try")
}
