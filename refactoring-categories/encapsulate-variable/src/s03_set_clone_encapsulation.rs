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

static DEFAULT_OWNER_DATA: LazyLock<Mutex<Owner>> = LazyLock::new(|| {
    Mutex::new(Owner {
        first_name: "Martin".to_string(),
        last_name: "Fowler".to_string(),
    })
});

fn default_owner() -> Owner {
    DEFAULT_OWNER_DATA.lock().expect("mutex poisoned").clone()
}

// Setter receives a copy of the data: in Java/JS we need explicit cloning to
// prevent source data modification issues But in Rust, due to ownership move
// semantics, the setter naturally receives a copy
fn set_default_owner(arg: Owner) {
    let mut owner = DEFAULT_OWNER_DATA.lock().expect("mutex poisoned");
    *owner = arg.clone(); // Explicitly clone to demonstrate the concept
}

fn update_default_owner_first_name(new_first_name: String) {
    let mut owner = DEFAULT_OWNER_DATA.lock().expect("mutex poisoned");
    owner.first_name = new_first_name;
}

fn update_default_owner_last_name(new_last_name: String) {
    let mut owner = DEFAULT_OWNER_DATA.lock().expect("mutex poisoned");
    owner.last_name = new_last_name;
}
#[allow(dead_code)]
fn update_default_owner_names(first_name: String, last_name: String) {
    let mut owner = DEFAULT_OWNER_DATA.lock().expect("mutex poisoned");
    owner.first_name = first_name;
    owner.last_name = last_name;
}

pub fn read_public_mutable_shared_state() -> String {
    let mut spaceship = Spaceship {
        owner: default_owner(),
    };

    spaceship.owner.first_name = "Modified".to_string();

    update_default_owner_first_name("Modified".to_string());
    update_default_owner_last_name("Content".to_string());

    set_default_owner(Owner {
        first_name: "Rebecca".to_string(),
        last_name: "Parsons".to_string(),
    });

    spaceship.owner = default_owner();

    spaceship.to_string()
}
