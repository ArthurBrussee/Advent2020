use std::fs;

fn crack(nums: &Vec<i64>, preamble: usize) -> Option<i64> {
    for i in preamble..nums.len() {
        let mut found = false;
        'outer: for j in i - preamble..i {
            for k in j + 1..i {
                if nums[j] + nums[k] == nums[i] {
                    found = true;
                    break 'outer;
                }
            }
        }
        if !found {
            return Some(nums[i]);
        }
    }

    None
}

pub fn calc() -> (i64, i64) {
    let nums = fs::read_to_string("./inputs/day9.txt")
        .expect("Can't find input file.")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    let p1 = crack(&nums, 25).unwrap();

    let sat = nums
        .iter()
        .scan(0, |sum, i| Some(*sum + i))
        .collect::<Vec<_>>();

    for i in 0..sat.len() {
        for j in i..sat.len() {
            if sat[j] - sat[i] == p1 {
                let slice = &nums[i + 1..j + 1];
                let p2 = slice.iter().min().unwrap() + slice.iter().max().unwrap();
                return (p1, p2);
            }
        }
    }
    unreachable!("No solution");
}
