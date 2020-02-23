pub fn insertion_sort<T: PartialOrd>(v: &mut Vec<T>) -> &Vec<T> {
    if v.len() <= 1 {
        return v;
    }

    for i in 1..v.len() {
        for j in (1..=i).rev() {
            // compare v[j] with v[j-1]
            if v[j] < v[j-1] {
                v.swap(j, j-1);
            } else {
                break
            }
        }
    }

    v
}

// rearrange and produce "partially" sorted array that can eventually be sorted by insertion sort
pub fn shell_sort<T: PartialOrd>(v: &mut Vec<T>) -> &Vec<T> {
    if v.len() <= 1 {
        return v;
    }

    let mut h = 1;
    while h < v.len()/3 {
        h = 3*h+1;
    }

    // when h == 1, shell sort becomes insertion sort
    while h >= 1 {
        for i in h..v.len() {
            for j in (h..=i).rev().step_by(h) {
                if v[j] < v[j-h] {
                    v.swap(j, j-h);
                } else {
                    break;
                }
            }
        }

        h /= 3
    }

    v
}
