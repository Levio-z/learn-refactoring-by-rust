mod s01_before_encapsulation;
mod s02_after_encapsulation;

fn main() {
    s01_before_encapsulation::read_public_mutable_shared_state();
    s02_after_encapsulation::read_public_mutable_shared_state();
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works_0() {
        // test print content
        assert_eq!(
            s01_before_encapsulation::read_public_mutable_shared_state(),
            "Spaceship owned by Rebecca Parsons"
        );
    }
    #[test]
    fn it_works_1() {
        // test print content
        assert_eq!(
            s02_after_encapsulation::read_public_mutable_shared_state(),
            "Spaceship owned by Rebecca Parsons"
        );
    }
}
