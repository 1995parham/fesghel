use rand::{distributions::Alphanumeric, thread_rng, Rng};

struct URL {}

impl URL {
    pub fn random_key() -> String {
        thread_rng().sample_iter(&Alphanumeric).take(6).collect()
    }
}

mod tests {
    #[test]
    fn random_key() {
        let s1 = super::URL::random_key();
        assert_eq!(s1.len(), 6);

        let s2 = super::URL::random_key();

        assert_ne!(s1, s2);
    }
}
