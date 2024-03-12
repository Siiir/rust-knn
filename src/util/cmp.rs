pub mod by_dist_sq {
    use derive_more::Constructor;

    #[derive(Constructor)]
    pub struct ComparedByDistSq<T> {
        pub val: T,
        pub dist_sq: f32,
    }
    impl<T> PartialEq for ComparedByDistSq<T> {
        fn eq(&self, other: &Self) -> bool {
            self.dist_sq == other.dist_sq
        }
    }
    impl<T> PartialOrd for ComparedByDistSq<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.dist_sq.partial_cmp(&other.dist_sq)
        }
    }
    impl<T> Eq for ComparedByDistSq<T> {}
    impl<T> Ord for ComparedByDistSq<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
}
