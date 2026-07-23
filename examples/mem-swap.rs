#[expect(clippy::absolute_paths, reason = "explicitness")]
fn main() {
    type Job = String;
    struct Machine {
        active: Vec<Job>,
        pending: Vec<Job>,
    }
    let mut mc = Machine {
        active: Vec::new(),
        pending: Vec::new(),
    };
    mc.pending.push(Job::from("payroll"));
    // Flip the buffers: pending jobs become active
    std::mem::swap(&mut mc.active, &mut mc.pending);
    println!("{:?}", mc.active.first());
    // Some("payroll")
}
