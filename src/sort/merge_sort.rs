pub fn merge_sort_basic<T: PartialOrd + Copy>(v: &mut Vec<T>) -> &Vec<T> {
    let mut aux = v.clone();
    sort(v, &mut aux, 0, v.len()-1);
    v
}

fn sort<T: PartialOrd + Copy>(v: &mut Vec<T>, aux: &mut Vec<T>, lo: usize, hi: usize) {
    if lo >= hi {
        return
    }

    let mid = lo + (hi - lo) / 2;
    sort(v, aux, lo, mid);
    sort(v, aux, mid+1, hi);
    merge(v, aux, lo, mid, hi);
}

fn merge<T: PartialOrd + Copy>(v: &mut Vec<T>, aux: &mut Vec<T>, lo: usize, mid: usize, hi: usize) {
    // prerequisite: both lo..=mid and mid+1..=hi are sorted
    // copy
    for i in lo..=hi {
        aux[i] = v[i];
    }

    let mut i = lo;
    let mut j = mid+1;
    for k in lo..=hi {
        if i > mid {
            v[k] = aux[j];
            j += 1
        } else if j > hi {
            v[k] = aux[i];
            i += 1
        } else if aux[i] > aux[j] {
            v[k] = aux[j];
            j += 1;
        } else {
            v[k] = aux[i];
            i += 1;
        }
    }
}
