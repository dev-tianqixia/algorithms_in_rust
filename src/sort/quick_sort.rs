use rand::Rng;

pub fn quick_sort(v: &mut Vec<isize>) {
    shuffle(v);
    sort(v, 0, v.len()-1);
}

fn shuffle(v: &mut Vec<isize>) {
    let mut gen = rand::thread_rng();
    for i in 0..v.len() {
        v.swap(i, gen.gen_range(0, i+1));
    }
}

fn sort(v: &mut Vec<isize>, lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let mid = partition(v, lo, hi);

    if mid > 0 {
        sort(v, lo, mid-1);
    }
    sort(v, mid+1, hi);
}

fn partition(v: &mut Vec<isize>, lo: usize, hi: usize) -> usize {
    let pivot = v[lo];
    let mut i = lo+1;
    let mut j = hi;

    loop {
        while i <= hi && v[i] < pivot {
            i += 1;
        }
        
        while j > lo && v[j] > pivot {
            j -= 1;
        }

        if i >= j {
            break;
        }

        v.swap(i, j);
        i += 1;
        j -= 1;
    }

    v.swap(lo, j);
    j
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sort_and_assert(v: &mut Vec<isize>) {
        quick_sort(v);
        for i in 0..v.len()-1 {
            assert!(v[i] <= v[i+1]);
        }
    }

    #[test]
    fn test_quick_sort() {
        sort_and_assert(&mut vec![4,2,6,7,1,10]);
        sort_and_assert(&mut vec![1,1,1,1,1]);
    }
}