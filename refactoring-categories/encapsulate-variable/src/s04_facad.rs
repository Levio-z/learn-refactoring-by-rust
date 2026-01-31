use std::sync::{LazyLock, Mutex};

#[derive(Clone, Debug)]
struct Owner {
    first_name: String,
    last_name: String,
}
impl std::fmt::Display for Owner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}

#[derive(Debug)]
struct Spaceship {
    owner: Mutex<Owner>,
}
impl Spaceship {
    fn new() -> Spaceship {
        Spaceship {
            owner: Mutex::new(Owner {
                first_name: "Martin".to_string(),
                last_name: "Fowler".to_string(),
            }),
        }
    }

    // This method can easily lead to deadlocks as it directly returns a reference
    // pub fn get_ref(&self) -> std::sync::MutexGuard<'_, Owner> {
    //     self.owner.lock().expect("mutex poisoned")
    // }
    // Provide a method to access the owner reference with a clear semantic
    pub fn with_owner_ref<R>(&self, f: impl FnOnce(&Owner) -> R) -> R {
        let guard = self.owner.lock().unwrap();
        f(&guard)
    }

    // Provide a method to access the owner mutable reference with a clear semantic
    fn with_owner_mut<R>(&self, f: impl FnOnce(&mut Owner) -> R) -> R {
        let mut guard = self.owner.lock().unwrap();
        println!("{:?}", guard);
        f(&mut guard)
    }
    pub fn get_clone(&self) -> Owner {
        self.owner.lock().expect("mutex poisoned").clone()
    }
    // Setter receives a copy of the data: in Java/JS we need explicit cloning to
    // prevent source data modification issues But in Rust, due to ownership move
    // semantics, the setter naturally receives a copy
    pub fn set_default_owner(&self, arg: Owner) {
        self.with_owner_mut(|owner| {
            *owner = arg;
        });
    }

    pub fn update_default_owner_first_name(&self, new_first_name: String) {
        self.with_owner_mut(|owner| {
            owner.first_name = new_first_name;
        });
    }

    pub fn update_default_owner_last_name(&self, new_last_name: String) {
        self.with_owner_mut(|owner| {
            owner.last_name = new_last_name;
        });
    }
    #[allow(dead_code)]
    pub fn update_default_owner_names(&self, first_name: String, last_name: String) {
        self.with_owner_mut(|owner| {
            owner.first_name = first_name;
            owner.last_name = last_name;
        });
    }
}
impl std::fmt::Display for Spaceship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Spaceship owned by {}",
            self.owner.lock().expect("mutex poisoned")
        )
    }
}

static DEFAULT_OWNER_DATA: LazyLock<Spaceship> = LazyLock::new(Spaceship::new);

pub fn read_public_mutable_shared_state() -> String {
    let mut owner = DEFAULT_OWNER_DATA.get_clone();
    owner.first_name = "Modified".to_string();

    DEFAULT_OWNER_DATA.with_owner_ref(|owner| print!("{}", owner));

    DEFAULT_OWNER_DATA.update_default_owner_first_name("Modified".to_string());
    DEFAULT_OWNER_DATA.update_default_owner_last_name("Content".to_string());

    DEFAULT_OWNER_DATA.set_default_owner(Owner {
        first_name: "Rebecca".to_string(),
        last_name: "Parsons".to_string(),
    });
    DEFAULT_OWNER_DATA.to_string()
}
