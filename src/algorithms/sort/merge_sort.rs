pub fn merge_sort(a: Vec<i32>) -> Vec<i32> {
    if a.len() < 2 {
        a.to_vec()
    } else {
        let c = merge_sort(a[0..a.len() / 2].to_vec());
        let d = merge_sort(a[a.len() / 2..].to_vec());
        merge(&c, &d)
    }
}
fn merge(c: &Vec<i32>, d: &Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut result = Vec::new();

    while i < c.len() && j < d.len() {
        if c[i] < d[j] {
            result.push(c[i]);
            i += 1;
        } else {
            result.push(d[j]);
            j += 1;
        }
    }
    if i < c.len() {
        while i < c.len() {
            result.push(c[i]);
            i += 1;
        }
    }
    if j < d.len() {
        while j < d.len() {
            result.push(d[j]);
            j += 1;
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};
    #[test]
    fn test_merge_sort() {
        let mut data = Vec::new();
        let mut rng = thread_rng();
        let data_len = rng.gen_range(10..100);
        for _i in 0..data_len {
            let x: i32 = rng.gen();
            data.push(x);
        }
        let got = merge_sort(data.clone());
        data.sort();
        assert_eq!(got, data);
    }
}
