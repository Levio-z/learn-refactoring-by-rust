use std::sync::{LazyLock, Mutex};

#[derive(Clone, Debug)]
struct Owner {
    first_name: String,
    last_name: String,
}

#[derive(Debug)]
struct Spaceship {
    owner: Owner,
}
impl std::fmt::Display for Spaceship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Spaceship owned by {} {}",
            self.owner.first_name, self.owner.last_name
        )
    }
}
// Solution 1: Encapsulate Variable with Mutable Reference
// This approach provides controlled mutable access to the shared state
// Global state is now private to this module.
// External code must go through accessor functions to modify the reference.
static DEFAULT_OWNER: LazyLock<Mutex<Owner>> = LazyLock::new(|| {
    Mutex::new(Owner {
        first_name: "Martin".to_string(),
        last_name: "Fowler".to_string(),
    })
});

// Read access to the default owner.
// Returns a mutable reference to allow content modification.
// This allows clients to modify the content but not the reference itself.
fn get_default_owner() -> std::sync::MutexGuard<'static, Owner> {
    DEFAULT_OWNER.lock().expect("mutex poisoned")
}

// Write access to the default owner reference.
// Only this function can change which Owner instance is referenced.
fn set_default_owner(new_owner: Owner) {
    let mut owner = DEFAULT_OWNER.lock().expect("mutex poisoned");
    *owner = new_owner;
}

pub fn read_public_mutable_shared_state() -> String {
    // Get mutable access to the owner content
    let mut spaceship = Spaceship {
        owner: get_default_owner().clone(),
    };

    // Client can modify the content of the owner
    {
        let mut owner = get_default_owner();
        owner.first_name = "Modified".to_string();
        owner.last_name = "Content".to_string();
    }

    // Update the default owner reference via the setter
    set_default_owner(Owner {
        first_name: "Rebecca".to_string(),
        last_name: "Parsons".to_string(),
    });

    // Re-read via the getter
    spaceship.owner = get_default_owner().clone();
    spaceship.to_string()
}
