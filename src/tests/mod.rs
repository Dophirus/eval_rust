#[cfg(test)]
mod tests {
    use crate::cache_lru::CacheLru;
    use crate::cache_lru_trait::CacheLruTrait;

    #[test]
    fn test_get () {
        let mut cache = CacheLru::new(3);
        cache.set("A", String::from("value A"));
        assert_eq!(cache.get("A").unwrap(), "value A");
        assert_eq!(cache.get("B"), None);
    }

    #[test]
    fn test_set () {
        let mut cache = CacheLru::new(3);
        cache.set("A", 1);
        assert_eq!(cache.get("A").unwrap(), &1);
        cache.set("A", 2);
        assert_eq!(cache.get("A").unwrap(), &2);

        cache.set("B", 1);
        cache.set("C", 1);
        cache.set("D", 1);
        assert_eq!(cache.get("B").unwrap(), &1);
        assert_eq!(cache.get("A"), None);
    }
}