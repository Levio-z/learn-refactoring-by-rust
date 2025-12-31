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
// Global mutable state exposed to all callers.
// Any code can read or modify it directly.
static DEFAULT_OWNER: LazyLock<Mutex<Owner>> = LazyLock::new(|| {
    Mutex::new(Owner {
        first_name: "Martin".to_string(),
        last_name: "Fowler".to_string(),
    })
});
// Callers depend on the concrete storage of the global state
// Read/write logic is scattered across the codebase
// No single place to enforce invariants or semantics
// Freely assigns defaultOwner
pub fn read_public_mutable_shared_state() -> String {
    // Directly read the global state
    let mut spaceship = Spaceship {
        owner: DEFAULT_OWNER.lock().unwrap().clone(),
    };

    // Directly mutate the global state from anywhere
    *DEFAULT_OWNER.lock().unwrap() = Owner {
        first_name: "Rebecca".to_string(),
        last_name: "Parsons".to_string(),
    };

    // Read again after mutation
    spaceship.owner = DEFAULT_OWNER.lock().unwrap().clone();

    spaceship.to_string()
}
