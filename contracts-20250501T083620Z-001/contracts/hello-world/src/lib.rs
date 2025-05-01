#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

// Structure to represent a collection event
#[contracttype]
#[derive(Clone)]
pub struct CollectionEvent {
    pub id: u64,
    pub collector: Address,
    pub description: String,
    pub weight_in_kg: i128,
}

// Keys for storage
#[contracttype]
pub enum CollectionKey {
    Event(u64),
    Count,
}

#[contract]
pub struct PlasticCollectionTracker;

#[contractimpl]
impl PlasticCollectionTracker {
    // Record a new plastic collection event
    pub fn record_collection(env: Env, collector: Address, description: String, weight_in_kg: i128) -> u64 {
        let mut count = env.storage().instance().get(&CollectionKey::Count).unwrap_or(0);
        count += 1;

        let event = CollectionEvent {
            id: count,
            collector,
            description,
            weight_in_kg,
        };

        env.storage().instance().set(&CollectionKey::Event(count), &event);
        env.storage().instance().set(&CollectionKey::Count, &count);

        count
    }

    // Retrieve a specific collection event by ID
    pub fn get_collection(env: Env, event_id: u64) -> CollectionEvent {
        env.storage().instance().get(&CollectionKey::Event(event_id)).expect("Event not found")
    }

    // List all recorded collection events
    pub fn list_collections(env: Env) -> Vec<CollectionEvent> {
        let mut list = Vec::new(&env);
        let mut count = env.storage().instance().get(&CollectionKey::Count).unwrap_or(0);

        while count > 0 {
            if let Some(event) = env.storage().instance().get(&CollectionKey::Event(count)) {
                list.insert((count as u32) - 1, event);
            }
            count -= 1;
        }

        list
    }
}
