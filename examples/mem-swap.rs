fn main() {
    use std::mem;
    struct Job(String);
    struct Machine {
        active: Vec<Job>,
        pending: Vec<Job>,
    }
    let mut m = Machine {
        active: Vec::new(),
        pending: Vec::new(),
    };
    m.pending.push(Job("payroll".into()));
    // Flip the buffers: pending jobs become active
    mem::swap(&mut m.active, &mut m.pending);
    println!("{:?}", m.active.first().unwrap().0);
}
