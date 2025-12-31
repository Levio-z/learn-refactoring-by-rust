use std::sync::{LazyLock, Mutex};

// Solution 2
// step 1: Before Clone Encapsulation,how to detect or modify
// modification attempts?
// ===============================================
// This approach uses type encapsulation to enforce immutability.Rust doesn't
// need this,rust can return immutable reference directly.
// ===============================================
// "Detecting and preventing modification of data structure internals is usually
// a temporary measure."
// "I create a new type wrapper that only provides read-only access to the
// underlying Owner data."
// "This way, the client code can't modify the owner field directly, only
// through the factory methods."
// "Afterwards I can remove these modification detection mechanisms, or provide
// appropriate modification functions."
// "Once all these are handled, I can modify the getter function to return a
// data copy."

#[derive(Debug, Clone)]
struct Owner {
    first_name: String,
    last_name: String,
}

// New type wrapper to enforce immutability
// This provides a read-only interface while keeping the original Owner
// structure unchanged
#[derive(Debug)]
struct ImmutableOwner {
    inner: Owner,
}

impl ImmutableOwner {
    fn new(owner: Owner) -> Self {
        Self { inner: owner }
    }

    // Read-only accessors
    // These provide immutable access to the underlying Owner data
    fn first_name(&self) -> &str {
        &self.inner.first_name
    }

    fn last_name(&self) -> &str {
        &self.inner.last_name
    }

    // No direct modification methods - only factory methods can create new
    // instances
}

#[derive(Debug)]
struct Spaceship {
    owner: ImmutableOwner,
}
impl std::fmt::Display for Spaceship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Spaceship owned by {} {}",
            self.owner.first_name(),
            self.owner.last_name()
        )
    }
}

static DEFAULT_OWNER: LazyLock<Mutex<Owner>> = LazyLock::new(|| {
    Mutex::new(Owner {
        first_name: "Martin".to_string(),
        last_name: "Fowler".to_string(),
    })
});

// Getter function returns the immutable wrapper
// This enforces immutability while keeping the original Owner structure
// unchanged Rust can return an immutable reference to find modification
// attempts at compile time
fn default_owner() -> ImmutableOwner {
    let owner = DEFAULT_OWNER.lock().expect("mutex poisoned").clone();
    ImmutableOwner::new(owner)
}

// Setter function for updating the shared state
// This is the appropriate modification function that can be provided
fn set_default_owner(arg: Owner) {
    let mut owner = DEFAULT_OWNER.lock().expect("mutex poisoned");
    *owner = arg;
}

pub fn read_public_mutable_shared_state() -> String {
    #[allow(unused_mut)]
    let mut spaceship = Spaceship {
        owner: default_owner(), // Returns immutable wrapper, client cannot modify internal data
    };

    // Client cannot modify internal data through the immutable wrapper
    // spaceship.owner.first_name = "Modified".to_string(); // This would cause
    // compilation error {
    //     let mut owner = default_owner();
    //     owner.first_name = "Modified".to_string();
    //     owner.last_name = "Content".to_string();
    // }

    // Update the default owner via the setter (appropriate modification function)
    set_default_owner(Owner {
        first_name: "Rebecca".to_string(),
        last_name: "Parsons".to_string(),
    });

    // Re-read to get the updated immutable wrapper
    spaceship.owner = default_owner();

    spaceship.to_string()
}
