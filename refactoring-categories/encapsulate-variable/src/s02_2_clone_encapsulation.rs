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

// Solution 2
// step 2: Clone Encapsulation with Appropriate Modification Functions
// This approach returns data copies to clients but also provides appropriate
// modification functions
// "Afterwards I can remove the modification detection mechanisms, or provide
// appropriate modification functions."
// "Once all these are handled, I can modify the getter function to return a
// data copy."

static DEFAULT_OWNER_DATA: LazyLock<Mutex<Owner>> = LazyLock::new(|| {
    Mutex::new(Owner {
        first_name: "Martin".to_string(),
        last_name: "Fowler".to_string(),
    })
});

// Getter function returns a data copy
// Clients receive a clone of the owner data
// This prevents modifications from affecting the shared state
fn default_owner() -> Owner {
    DEFAULT_OWNER_DATA.lock().expect("mutex poisoned").clone()
}

// Setter function for updating the shared state
// Only this function can modify the global owner reference
fn set_default_owner(arg: Owner) {
    let mut owner = DEFAULT_OWNER_DATA.lock().expect("mutex poisoned");
    *owner = arg;
}

// Appropriate modification functions for specific fields
// These functions provide safe ways to modify specific aspects of the owner
// data
fn update_default_owner_first_name(new_first_name: String) {
    let mut owner = DEFAULT_OWNER_DATA.lock().expect("mutex poisoned");
    owner.first_name = new_first_name;
}

fn update_default_owner_last_name(new_last_name: String) {
    let mut owner = DEFAULT_OWNER_DATA.lock().expect("mutex poisoned");
    owner.last_name = new_last_name;
}

// Function to update both names at once
fn update_default_owner_names(first_name: String, last_name: String) {
    let mut owner = DEFAULT_OWNER_DATA.lock().expect("mutex poisoned");
    owner.first_name = first_name;
    owner.last_name = last_name;
}

pub fn read_public_mutable_shared_state() -> String {
    let mut spaceship = Spaceship {
        owner: default_owner(), // Returns a copy, client modifications won't affect shared data
    };

    // Client can modify the copy but it won't affect the original data
    spaceship.owner.first_name = "Modified".to_string();

    // Use appropriate modification functions to update the shared state
    // Instead of trying to modify the copy, use the provided functions
    update_default_owner_first_name("Modified".to_string());
    update_default_owner_last_name("Content".to_string());
    update_default_owner_names("Modified".to_string(), "Content".to_string());

    // Update the default owner via the setter (complete replacement)
    set_default_owner(Owner {
        first_name: "Rebecca".to_string(),
        last_name: "Parsons".to_string(),
    });

    // Re-read via the getter to get the updated data copy
    spaceship.owner = default_owner();

    spaceship.to_string()
}
