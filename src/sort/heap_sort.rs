pub fn heap_sort<T: PartialOrd>(v: &mut Vec<T>) -> &Vec<T> {
    if v.len() <= 1 {
        return v;
    }

    // build the binary heap from v
    for i in (0..v.len()/2).rev() {
        sink(v, i, v.len())
    }

    // invariant:
    // [end+1..v.len()]: sorted;
    // [0..=end]:        to be examined.
    let mut end = v.len()-1;
    while end > 0 {
        v.swap(0, end);
        sink(v, 0, end);
        end -= 1;
    }

    v
}

fn sink<T: PartialOrd>(v: &mut Vec<T>, start: usize, end: usize) {
    let mut i = start;
    while i < end/2 {
        let left = i*2+1;
        let right = i*2+2;
        let j = if right < end && v[right] > v[left] {
            right
        } else {
            left
        };

        if v[i] >= v[j] {
            break;
        }

        v.swap(i, j);
        i = j;
    }
}