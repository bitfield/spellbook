fn main() {
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
    std::mem::swap(&mut m.active, &mut m.pending);
    println!("{:?}", m.active.first());
}
