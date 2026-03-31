use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

// Singleton of Database for Subscribers grouped by Product Type
lazy_static! {
    static ref SUBSCRIBERS: DashMap<String, Vec<Subscriber>> = DashMap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository {
    pub fn add(product_type: &str, subscriber: Subscriber) -> Subscriber {
        let key = String::from(product_type);
        let mut subscribers = SUBSCRIBERS.entry(key).or_insert_with(Vec::new);
        subscribers.push(subscriber.clone());
        return subscriber;
    }

    pub fn list_all(product_type: &str) -> Vec<Subscriber> {
        let key = String::from(product_type);
        match SUBSCRIBERS.get(&key) {
            Some(subs) => subs.clone(),
            None => Vec::new(),
        }
    }

    pub fn delete(product_type: &str, url: &str) -> Option<Subscriber> {
        let key = String::from(product_type);
        if let Some(mut subscribers) = SUBSCRIBERS.get_mut(&key) {
            if let Some(pos) = subscribers.iter().position(|s| s.url == url) {
                return Some(subscribers.remove(pos));
            }
        }
        None
    }
}
