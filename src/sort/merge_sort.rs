use std::cmp;

pub fn merge_sort_top_down<T: PartialOrd + Copy>(v: &mut Vec<T>) -> &Vec<T> {
    let mut aux = v.clone();
    sort(v, &mut aux, 0, v.len()-1);
    v
}

pub fn merge_sort_bottom_up<T: PartialOrd + Copy>(v: &mut Vec<T>) -> &Vec<T> {
    let mut aux = v.clone();
    // first try to merge subarrays with size 1
    let mut size = 1;

    // do not use size <= v.len()/2 as the loop termination condition;
    // size < v.len() works for both odd/even length.
    while size < v.len() {
        // (v.len()-1 - lo)+1 > size -> lo < v.len() - size
        // v.len()-1: last index
        // lo: current lower bound
        // if the length from lo to the last index is less than or equal to size, a merge won't be necessary
        for lo in (0..v.len()-size).step_by(size*2) {
            let mid = lo + size - 1;
            let hi = cmp::min(lo + 2*size -1, v.len()-1);
            merge(v, &mut aux, lo, mid, hi);
        }
        size *= 2;
    }

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

// prerequisite: both lo..=mid and mid+1..=hi are sorted
fn merge<T: PartialOrd + Copy>(v: &mut Vec<T>, aux: &mut Vec<T>, lo: usize, mid: usize, hi: usize) {
    // skip if v[lo..=hi] is in sorted order
    if v[mid] <= v[mid+1] {
        return
    }

    // improvement: use two invocations of the sort method;
    // one that takes its input from the given array and puts the sorted output in the auxiliary array; 
    // the other takes its input from the auxiliary array and puts the sorted output in the given array.
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
