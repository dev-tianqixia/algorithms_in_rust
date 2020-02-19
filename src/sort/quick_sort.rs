use rand::Rng;

pub fn quick_sort_basic<T: PartialOrd>(v: &mut Vec<T>) -> &Vec<T> {
    shuffle(v);
    sort_basic(v, 0, v.len()-1);
    v
}

pub fn quick_sort_3way_partition<T: PartialOrd>(v: &mut Vec<T>) -> &Vec<T> {
    shuffle(v);
    sort_3way_partition(v, 0, v.len()-1);
    v
}

fn shuffle<T>(v: &mut Vec<T>) {
    let mut gen = rand::thread_rng();
    for i in 0..v.len() {
        v.swap(i, gen.gen_range(0, i+1));
    }
}

fn sort_basic<T: PartialOrd>(v: &mut Vec<T>, lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let mid = partition(v, lo, hi);

    if mid > 0 {
        sort_basic(v, lo, mid-1);
    }
    sort_basic(v, mid+1, hi);
}

// 3-way partitioning quick sort is optimized for vectors with a lot of duplicate entries.
// invariant:
//   v[lo..lt]    - less than pivot;
//   v[lt..i]     - equal to pivot(v[lt] == pivot);
//   v[i..=gt]    - to be examined(including gt);
//   v[gt+1..=hi] - larger than pivot
fn sort_3way_partition<T: PartialOrd>(v: &mut Vec<T>, lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    // let pivot = v[lo];
    let mut i = lo+1;
    let mut lt = lo;
    let mut gt = hi;
    
    while i <= gt {
        if v[i] < v[lo] {
            v.swap(i, lt);
            i += 1;
            lt += 1;
        } else if v[i] > v[lo] {
            v.swap(i, gt);
            gt -= 1;
        } else /* v[i] == v[lo] */ {
            i += 1;
        }
    }

    sort_3way_partition(v, lo, lt);
    sort_3way_partition(v, gt+1, hi);
}

// invariant: 
fn partition<T: PartialOrd>(v: &mut Vec<T>, lo: usize, hi: usize) -> usize {
    // let pivot = v[lo];
    let mut i = lo+1;
    let mut j = hi;

    loop {
        while i <= hi && v[i] < v[lo] {
            i += 1;
        }
        
        while j > lo && v[j] > v[lo] {
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
