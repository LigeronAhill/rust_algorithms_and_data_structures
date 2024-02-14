pub fn insertion_sort(mut a: Vec<i32>) -> Vec<i32> {
    for i in 1..a.len() {
        let mut j = i;
        while j > 0 && a[j - 1] > a[j] {
            a.swap(j - 1, j);
            j -= 1;
        }
    }
    a.to_vec()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};
    #[test]
    fn test_insertion_sort() {
        let mut data = Vec::new();
        let mut rng = thread_rng();
        let data_len = rng.gen_range(10..100);
        for _i in 0..data_len {
            let x: i32 = rng.gen();
            data.push(x);
        }
        let got = insertion_sort(data.clone());
        data.sort();
        assert_eq!(got, data);
    }
}
