use std::{collections::HashMap, hash::Hash};
use crate::cache_lru_trait::CacheLruTrait;

/// Un cache LRU qui stocke des éléments en fonction de sa taille maximale
pub struct CacheLru<K, T> {
    max_size: usize,
    data: HashMap<K, T>,
    order: Vec<K>
}

impl<K, T> CacheLru<K, T> where K:Clone + Hash + Eq {
    /// Crée un nouveau cache LRU avec une taille maximale spécifiée
    ///
    /// # Arguments
    ///
    /// * `max_size` - La taille maximale du cache
    ///
    /// # Exemple
    ///
    /// ```
    /// # use eval_rust::cache_lru::CacheLru;
    /// let mut cache: CacheLru<&str, String> = CacheLru::new(5);
    /// ```
    pub fn new(max_size: usize) -> Self {
        CacheLru {
            max_size,
            data: HashMap::new(),
            order: Vec::new(),
        }
    }
}

impl<K, T> CacheLruTrait<K, T> for CacheLru<K, T> where K:Clone + Hash + Eq {
    /// Ajoute une clé et une valeur au cache
    ///
    /// Si la clé existe déjà, elle est mise à jour et son ordre est réinitialisé
    /// Si le cache atteint sa taille maximale, l'élément le moins récemment utilisé est supprimé
    ///
    /// # Arguments
    ///
    /// * `key` - La clé à ajouter ou à mettre à jour
    /// * `value` - La valeur associée à la clé
    fn set(&mut self, key: K, value: T) {
        if self.data.contains_key(&key) {
            for (i, v) in self.order.clone().iter().enumerate() {
                if v == &key {
                    self.order.remove(i);
                }
            }
        } else if self.data.len() >= self.max_size {
            let deleted = self.order.remove(0);
            self.data.remove(&deleted);
        } 

        if self.data.len() <= self.max_size {
            self.data.insert(key.clone(), value);
            self.order.push(key);
        }
    }

    /// Récupère une référence à la valeur associée à la clé spécifiée
    ///
    /// Si la clé existe, elle est déplacée à la fin de l'ordre
    ///
    /// # Arguments
    ///
    /// * `key` - La clé dont la valeur doit être récupérée
    ///
    /// # Retourne
    ///
    /// Une option contenant une référence à la valeur si la clé existe, sinon `None`
    fn get(&mut self, key: K) -> Option<&T> {
        if self.data.contains_key(&key) {
            for (i, v) in self.order.clone().iter().enumerate() {
                if v == &key {
                    self.order.remove(i);
                    self.order.push(key.clone());
                }
            }
            return self.data.get(&key);
        } else {
            return None;
        }
    }
}

