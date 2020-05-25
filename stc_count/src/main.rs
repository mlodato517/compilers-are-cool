static STEPS: [usize; 5] = [3, 3, 2, 0, 0];

fn num_valid_walks(mut position: usize) -> usize {
    if position == STEPS.len() - 1 {
        1
    } else if position >= STEPS.len() || STEPS[position] == 0 {
        0
    } else {
        let mut count = 0;

        let max_step = STEPS[position];
        for step in 1..=max_step {
            position += step;
            count += num_valid_walks(position);
            position -= step;
        }

        count
    }
}

fn main() {
    println!("{:?}", num_valid_walks(0));
}
