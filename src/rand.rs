use rand::Rng;

pub fn sample<'a, V>(mut rng: impl Rng, vs: &'a [V]) -> Option<&'a V> {
    if vs.is_empty() {
        None
    } else {
        Some(&vs[rng.gen_range(0, vs.len())])
    }
}

#[cfg(test)]
mod spec {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn sample_empty_vector() {
        let mut rng = thread_rng();

        let vs: Vec<String> = vec![];

        assert_eq!(sample(rng, &vs), None)
    }

    #[test]
    fn sample_nonempty_vector() {
        let mut rng = thread_rng();
        let vs: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        for _ in 1..100 {
            match sample(rng, &vs) {
                Some(item) => assert!(vs.contains(item)),
                None => panic!("should always return item"),
            }
        }
    }
}
