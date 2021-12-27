use std::collections::{HashMap, BinaryHeap};



// create a function that is a priority queue and is backed by a key value store 
// that is a hashmap of key value pairs 
// the key is the priority and the value is the value
// the key value pair is a tuple
// the priority is a number
// the value is a string

fn priority_queue() {
    let mut key_value_store = HashMap::new();

    key_value_store.insert(1, "first");
    key_value_store.insert(2, "second");
    key_value_store.insert(3, "third");
    key_value_store.insert(4, "fourth");
    key_value_store.insert(5, "fifth");
    key_value_store.insert(6, "sixth");

    let mut priority_queue = BinaryHeap::new();

    for (key, value) in key_value_store.iter() {
        priority_queue.push(PriorityQueueItem {
            priority: *key,
            value: value.to_string()
        });
    }

    while let Some(item) = priority_queue.pop() {
        println!("{}", item.value);
    }
}

#[derive(Debug)]
struct PriorityQueueItem {
    priority: i32,
    value: String
}

impl PartialEq for PriorityQueueItem {
    fn eq(&self, other: &PriorityQueueItem) -> bool {
        self.priority == other.priority
    }
}

impl Eq for PriorityQueueItem {}

impl PartialOrd for PriorityQueueItem {
    fn partial_cmp(&self, other: &PriorityQueueItem) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriorityQueueItem {
    fn cmp(&self, other: &PriorityQueueItem) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

fn main() {
    priority_queue();
}