fn main() {
    let grid = vec![
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 0],
    ];
    'outer: for row in grid {
        for col in row {
            if col == 1 {
                println!("Found it!");
                break 'outer;
            }
        }
    }
}
