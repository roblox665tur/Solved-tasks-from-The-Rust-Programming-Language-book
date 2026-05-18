use std::collections::HashMap;

fn median_value(vec: &mut [isize]) -> Option<f32> {
    if vec.is_empty() {
        return None;
    }

    vec.sort();
    let mid = vec.len();
    if vec.len() % 2 == 0 {
        Some((vec[mid / 2] + vec[mid / 2 - 1]) as f32 / 2.0)
    } else {
        Some(vec[mid / 2] as f32)
    }
}

fn mode_of_vector(vec: &[isize]) -> Option<(isize, isize)> {
    if vec.is_empty() {
        return None;
    }

    let mut mode_list: HashMap<isize, isize> = HashMap::new();

    for &x in vec.iter() {
        let mut count = mode_list.entry(x).or_insert(0);
        *count += 1;
    }
    vec.iter()
        .map(|&k| (k, mode_list[&k]))
        .max_by_key(|&(_, y)| y)
}
