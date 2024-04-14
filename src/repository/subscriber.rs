use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

// Singleton of Database
lazy_static! {
    static ref SUBSCRIBERS: DashMap<String, DashMap<String
}

pub struct SubscriberRepository;

impl SubscriberRepository {
}