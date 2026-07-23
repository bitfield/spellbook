#[derive(Default)]
struct SausageMachine(usize);

impl Iterator for SausageMachine {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = self.0.wrapping_add(1);
        Some(format!("Sausage no.{}", self.0))
    }
}

fn main() {
    for sausage in SausageMachine::default().take(3) {
        println!("{sausage}");
    }
    // Sausage no.1
    // Sausage no.2
    // Sausage no.3
}
