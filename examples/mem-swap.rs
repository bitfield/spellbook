fn main() {
    use std::mem;
    type Job = String;
    struct Machine {
        active: Vec<Job>,
        pending: Vec<Job>,
    }
    let mut m = Machine {
        active: Vec::new(),
        pending: Vec::new(),
    };
    m.pending.push(Job::from("payroll"));
    // Flip the buffers: pending jobs become active
    mem::swap(&mut m.active, &mut m.pending);
    println!("{:?}", m.active.first());
}
