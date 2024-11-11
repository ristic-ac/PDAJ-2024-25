use std::cell::RefCell;
use std::rc::Rc;

// Trait for message-sending functionality
pub trait Messenger {
    fn send(&self, msg: &str);
}

// QuotaTracker struct that tracks usage and sends warnings via a Messenger
pub struct QuotaTracker<'a, T: Messenger> {
    messenger: &'a T,
    current_usage: usize,
    quota_limit: usize,
}

impl<'a, T> QuotaTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, quota_limit: usize) -> QuotaTracker<'a, T> {
        QuotaTracker {
            messenger,
            current_usage: 0,
            quota_limit,
        }
    }

    // Method to increment the usage count
    pub fn use_quota(&mut self, amount: usize) {
        self.current_usage += amount;
        let usage_percentage = self.current_usage as f64 / self.quota_limit as f64;

        // Send messages based on usage thresholds
        if usage_percentage >= 1.0 {
            self.messenger.send("Error: Quota exceeded!");
        } else if usage_percentage >= 0.9 {
            self.messenger.send("Urgent: Over 90% of quota used!");
        } else if usage_percentage >= 0.75 {
            self.messenger.send("Warning: Over 75% of quota used!");
        }
    }
}

// MockMessenger for capturing messages instead of sending them
struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }

    // Method to retrieve sent messages
    fn show_messages(&self) {
        for (index, message) in self.sent_messages.borrow().iter().enumerate() {
            println!("Message {}: {}", index + 1, message);
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        // Using RefCell to mutate the vector of messages
        self.sent_messages.borrow_mut().push(String::from(msg));
    }
}

// Main function to demonstrate usage
fn main() {
    // Set up a mock messenger and quota tracker
    let mock_messenger = MockMessenger::new();
    let mut quota_tracker = QuotaTracker::new(&mock_messenger, 100);

    // Simulate usage increments and see the messages sent
    quota_tracker.use_quota(50);
    quota_tracker.use_quota(30);
    quota_tracker.use_quota(10);
    quota_tracker.use_quota(15);

    // Display all messages sent during the quota tracking
    println!("\nMessages sent during tracking:");
    mock_messenger.show_messages();

    // Demonstrate shared ownership and mutability with Rc and RefCell
    println!("\nShared Quota Example:");

    // Create a shared quota using Rc and RefCell
    let shared_quota = Rc::new(RefCell::new(100));

    // Clone the Rc<T> references to share ownership of the quota
    let quota1 = Rc::clone(&shared_quota);
    let quota2 = Rc::clone(&shared_quota);

    // Mutate the quota from different owners
    // Important note: Non-lexical lifetimes do not apply to this example
    let mut quota1_borrowed = quota1.borrow_mut();
    *quota1_borrowed -= 30; // Reduce quota by 30
    // Borrowing here while the quota is still borrowed mutably
    // This would cause a panic at runtime due to a double mutable borrow
    let mut quota2_borrowed = quota2.borrow_mut();
    *quota2_borrowed -= 20; // Reduce quota by another 20


    // Increase the quota without storing the borrowed value
    *quota1.borrow_mut() += 30; // Increase quota by 30
    *quota2.borrow_mut() += 20; // Increase quota by 20


    println!("Quota after first reduction: {}", *shared_quota.borrow());
    println!("Quota after second reduction: {}", *shared_quota.borrow());

    // Final value of the shared quota
    println!("Final quota value: {}", *shared_quota.borrow());
}
