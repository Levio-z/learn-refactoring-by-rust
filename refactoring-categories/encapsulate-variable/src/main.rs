mod s00_before_encapsulation;
mod s01_after_encapsulation;
mod s02_1_find_modify;
mod s02_2_clone_encapsulation;
mod s03_set_clone_encapsulation;

fn main() {
    s00_before_encapsulation::read_public_mutable_shared_state();
    s01_after_encapsulation::read_public_mutable_shared_state();
    s02_1_find_modify::read_public_mutable_shared_state();
    s02_2_clone_encapsulation::read_public_mutable_shared_state();
    s03_set_clone_encapsulation::read_public_mutable_shared_state();
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_0() {
        // test print content
        assert_eq!(
            s00_before_encapsulation::read_public_mutable_shared_state(),
            "Spaceship owned by Rebecca Parsons"
        );
    }

    #[test]
    fn it_works_1() {
        // test print content
        assert_eq!(
            s01_after_encapsulation::read_public_mutable_shared_state(),
            "Spaceship owned by Rebecca Parsons"
        );
    }

    #[test]
    fn it_works_2() {
        // test print content
        assert_eq!(
            s02_1_find_modify::read_public_mutable_shared_state(),
            "Spaceship owned by Rebecca Parsons"
        );
        assert_eq!(
            s02_2_clone_encapsulation::read_public_mutable_shared_state(),
            "Spaceship owned by Rebecca Parsons"
        );
    }

    #[test]
    fn it_works_3() {
        // test print content
        assert_eq!(
            s03_set_clone_encapsulation::read_public_mutable_shared_state(),
            "Spaceship owned by Rebecca Parsons"
        );
    }
}
