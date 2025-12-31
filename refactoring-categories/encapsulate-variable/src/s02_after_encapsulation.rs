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
// Global state is now private to this module.
// External code must go through accessor functions.
static DEFAULT_OWNER: LazyLock<Mutex<Owner>> = LazyLock::new(|| {
    Mutex::new(Owner {
        first_name: "Martin".to_string(),
        last_name: "Fowler".to_string(),
    })
});

// Read access to the default owner.
// Returns a cloned value to avoid exposing internal mutability.
fn get_default_owner() -> Owner {
    DEFAULT_OWNER.lock().expect("mutex poisoned").clone()
}

// Write access to the default owner.
// All mutations are centralized here.
fn set_default_owner(new_owner: Owner) {
    let mut owner = DEFAULT_OWNER.lock().expect("mutex poisoned");
    *owner = new_owner;
}

pub fn read_public_mutable_shared_state() -> String {
    let mut spaceship = Spaceship {
        owner: get_default_owner(),
    };

    // Update the default owner via the setter
    set_default_owner(Owner {
        first_name: "Rebecca".to_string(),
        last_name: "Parsons".to_string(),
    });

    // Re-read via the getter
    spaceship.owner = get_default_owner();
    spaceship.to_string()
}
