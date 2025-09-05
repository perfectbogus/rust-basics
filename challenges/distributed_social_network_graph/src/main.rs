// Advanced Challenge: Distributed Social Network Graph
// Combines: Graph Algorithms + Rc/RefCell/Weak + Data Structures
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque, HashSet};

// TODO: Your implementation goes here!
struct SocialNetwork {
    users: RefCell<HashMap<String, Rc<RefCell<User>>>>,
}

impl SocialNetwork {
    fn new() -> Self {
        SocialNetwork {
            users: RefCell::new(HashMap::new()),
        }
    }

    fn create_user(&mut self, username: String) -> Rc<RefCell<User>> {
        let rc_user = Rc::new(RefCell::new(User::new(&username)));
        self.users.borrow_mut().insert(username, rc_user.clone());
        rc_user
    }


}


struct User {
    name: &str,
    friends: RefCell<Vec<Weak<RefCell<User>>>>,
    messages: RefCell<Vec<Rc<Message>>>,
    network: Weak<SocialNetwork>,
}

impl User {
    fn new(name: &str) -> Self {
        User {
            name,
            friends: RefCell::new(Vec::new()),
            messages: RefCell::new(Vec::new()),
            network: Weak::new(),
        }
    }
}

struct Message {
    content: String,
    timestamp: u64,
    author: Weak<RefCell<User>>
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation_and_friendship() {
        let mut network = SocialNetwork::new();

        let alice = network.create_user("Alice".to_string());
        let bob = network.create_user("Bob".to_string());
        let charlie = network.create_user("Charlie".to_string());

        // Create friendships
        network.add_friendship(&alice, &bob);
        network.add_friendship(&bob, &charlie);

        // Alice and Bob should be friends
        assert!(alice.is_friend_with(&bob));
        assert!(bob.is_friend_with(&alice));

        // Alice and Charlie should NOT be direct friends
        assert!(!alice.is_friend_with(&charlie));

        // But they should be connected through Bob (path length 2)
        let path = network.shortest_path(&alice, &charlie);
        assert_eq!(path.len(), 3); // Alice -> Bob -> Charlie
    }

    #[test]
    fn test_message_posting_and_timeline() {
        let mut network = SocialNetwork::new();

        let alice = network.create_user("Alice".to_string());
        let bob = network.create_user("Bob".to_string());
        network.add_friendship(&alice, &bob);

        // Alice posts a message
        alice.post_message("Hello World!".to_string());
        alice.post_message("Rust is awesome!".to_string());

        // Bob posts a message
        bob.post_message("I agree!".to_string());

        // Alice's timeline should show her own messages
        let alice_timeline = alice.get_timeline();
        assert_eq!(alice_timeline.len(), 2);
        assert!(alice_timeline[0].content.contains("Rust is awesome"));

        // Alice's feed should include Bob's messages too
        let alice_feed = alice.get_news_feed();
        assert!(alice_feed.len() >= 3); // Her 2 + Bob's 1
    }

    #[test]
    fn test_influence_propagation() {
        let mut network = SocialNetwork::new();

        // Create a network: Alice -> Bob -> Charlie -> David
        let alice = network.create_user("Alice".to_string());
        let bob = network.create_user("Bob".to_string());
        let charlie = network.create_user("Charlie".to_string());
        let david = network.create_user("David".to_string());

        network.add_friendship(&alice, &bob);
        network.add_friendship(&bob, &charlie);
        network.add_friendship(&charlie, &david);

        // Alice starts a viral message
        alice.post_message("Breaking news!".to_string());

        // Simulate influence propagation (viral spreading)
        let influenced_users = network.propagate_influence(&alice, 2); // 2 hops max

        // Should reach Bob and Charlie, but not David (too far)
        assert!(influenced_users.len() >= 2);
        // Note: Test should verify Bob and Charlie are influenced
    }

    #[test]
    fn test_mutual_followers_algorithm() {
        let mut network = SocialNetwork::new();

        let alice = network.create_user("Alice".to_string());
        let bob = network.create_user("Bob".to_string());
        let charlie = network.create_user("Charlie".to_string());
        let diana = network.create_user("Diana".to_string());

        // Create network:
        // Alice <-> Bob, Alice <-> Charlie, Bob <-> Charlie, Charlie <-> Diana
        network.add_friendship(&alice, &bob);
        network.add_friendship(&alice, &charlie);
        network.add_friendship(&bob, &charlie);
        network.add_friendship(&charlie, &diana);

        // Alice and Bob should have 1 mutual friend (Charlie)
        let mutual = network.find_mutual_friends(&alice, &bob);
        assert_eq!(mutual.len(), 1);

        // Alice and Diana should have 1 mutual friend (Charlie)
        let mutual2 = network.find_mutual_friends(&alice, &diana);
        assert_eq!(mutual2.len(), 1);
    }

    #[test]
    fn test_graph_analysis() {
        let mut network = SocialNetwork::new();

        // Create a more complex network
        let users: Vec<_> = (0..6).map(|i| {
            network.create_user(format!("User{}", i))
        }).collect();

        // Create connections: 0-1, 1-2, 2-3, 0-4, 4-5
        network.add_friendship(&users[0], &users[1]);
        network.add_friendship(&users[1], &users[2]);
        network.add_friendship(&users[2], &users[3]);
        network.add_friendship(&users[0], &users[4]);
        network.add_friendship(&users[4], &users[5]);

        // Find the most connected user (should be User0 or User1)
        let most_connected = network.find_most_connected_user();
        assert!(most_connected.is_some());

        // Check if the graph is connected
        assert!(network.is_connected());

        // Calculate clustering coefficient for User1
        let clustering = network.clustering_coefficient(&users[1]);
        assert!(clustering >= 0.0 && clustering <= 1.0);
    }

    #[test]
    fn test_memory_safety_and_cycles() {
        let mut network = SocialNetwork::new();

        let alice = network.create_user("Alice".to_string());
        let bob = network.create_user("Bob".to_string());

        network.add_friendship(&alice, &bob);

        // Create circular references (this should NOT cause memory leaks)
        alice.post_message("Hello Bob!".to_string());
        bob.post_message("Hello Alice!".to_string());

        // Drop network - everything should clean up properly
        drop(network);

        // Users should still be accessible but their network should be gone
        assert_eq!(alice.get_name(), "Alice");
        assert_eq!(bob.get_name(), "Bob");

        // But network-dependent operations should handle missing network gracefully
        let feed = alice.get_news_feed();
        // Should either work or handle gracefully, not panic
    }

    #[test]
    fn test_concurrent_access_simulation() {
        let mut network = SocialNetwork::new();

        let alice = network.create_user("Alice".to_string());
        let bob = network.create_user("Bob".to_string());

        network.add_friendship(&alice, &bob);

        // Simulate multiple "threads" accessing the same users
        let alice_clone = alice.clone();
        let bob_clone = bob.clone();

        // Both should be able to post simultaneously (in real single-threaded context)
        alice.post_message("Message 1".to_string());
        alice_clone.post_message("Message 2".to_string());
        bob.post_message("Bob's message".to_string());
        bob_clone.post_message("Bob's second message".to_string());

        // All messages should be visible
        let alice_timeline = alice.get_timeline();
        assert_eq!(alice_timeline.len(), 2);

        let bob_timeline = bob.get_timeline();
        assert_eq!(bob_timeline.len(), 2);
    }
}