/// Un trait pour un cache LRU qui définit les opérations de base (set, get)
pub trait CacheLruTrait<K, T> {
    /// Ajoute de nouvelles données dans le cache sous la clé spécifiée
    ///
    /// # Exemple
    ///
    /// ```
    /// # use eval_rust::cache_lru::CacheLru;
    /// # use eval_rust::cache_lru_trait::CacheLruTrait;
    /// let mut cache: CacheLru<&str, String> = CacheLru::new(3);
    /// cache.set("A", String::from("value_a"));
    /// ```
    ///
    /// Si le cache dépasse sa capacité, la paire clé/valeur la plus ancienne sera supprimée
    ///
    /// # Exemple
    ///
    /// ```
    /// # use eval_rust::cache_lru::CacheLru;
    /// # use eval_rust::cache_lru_trait::CacheLruTrait;
    /// let mut cache: CacheLru<&str, String> = CacheLru::new(3);
    /// cache.set("A", String::from("value_a")); // => [A]
    /// cache.set("B", String::from("value_b")); // => [A, B]
    /// cache.set("C", String::from("value_c")); // => [A, B, C]
    /// cache.set("D", String::from("value_d")); // => [B, C, D]
    /// ```
    fn set(&mut self, key: K, value: T);

    /// Récupère des données du cache en utilisant une clé spécifique
    ///
    /// Peut retourner `None` si la clé n'existe pas
    ///
    /// # Exemple
    ///
    /// ```
    /// # use eval_rust::cache_lru::CacheLru;
    /// # use eval_rust::cache_lru_trait::CacheLruTrait;
    /// let mut cache: CacheLru<&str, String> = CacheLru::new(3);
    /// cache.set("A", String::from("value_a"));
    /// let my_value = cache.get("A").unwrap();
    /// ```
    ///
    /// Récupérer une valeur "rafraîchit" son ordre de suppression prioritaire
    ///
    /// # Exemple
    ///
    /// ```
    /// # use eval_rust::cache_lru::CacheLru;
    /// # use eval_rust::cache_lru_trait::CacheLruTrait;
    /// let mut cache: CacheLru<&str, String> = CacheLru::new(3);
    /// cache.set("A", String::from("value_a")); // => [A]
    /// cache.set("B", String::from("value_b")); // => [A, B]
    /// cache.set("C", String::from("value_c")); // => [A, B, C]
    /// let my_value = cache.get("A").unwrap();  // => [B, C, A]
    /// ```
    fn get(&mut self, key: K) -> Option<&T>;
}