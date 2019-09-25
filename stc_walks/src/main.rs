static STEPS: [usize; 5] = [3, 3, 2, 0, 0];
type Walk = Vec<usize>;

fn valid_walks(walk: &mut Walk) -> Vec<Walk> {
    let position: usize = walk.iter().sum();
    if position == STEPS.len() - 1 {
        vec![walk.clone()]
    } else if
        position >= STEPS.len() ||
        STEPS[position] == 0
    {
        vec![]
    } else {
        let mut walks = Vec::new();

        let max_step = STEPS[position];
        for step in 1..=max_step {
            walk.push(step);
            walks.append(&mut valid_walks(walk));
            walk.pop();
        }

        walks
    }
}

fn main() {
    let mut walk = Vec::new();
    println!("{:?}", valid_walks(&mut walk));
}
