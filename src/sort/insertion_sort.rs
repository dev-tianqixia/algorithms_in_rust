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
