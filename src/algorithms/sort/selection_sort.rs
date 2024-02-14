pub fn selection_sort(mut a: Vec<i32>) -> Vec<i32> {
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            if a[j] < a[i] {
                a.swap(i, j);
            }
        }
    }
    a.to_vec()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};
    #[test]
    fn test_selection_sort() {
        let mut data = Vec::new();
        let mut rng = thread_rng();
        let data_len = rng.gen_range(10..100);
        for _i in 0..data_len {
            let x: i32 = rng.gen();
            data.push(x);
        }
        let got = selection_sort(data.clone());
        data.sort();
        assert_eq!(got, data);
    }
}
