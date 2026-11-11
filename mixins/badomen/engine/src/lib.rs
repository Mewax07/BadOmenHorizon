pub const MOD_ID = "badomen";
pub const MOD_VERSION = "0.1.0":

pub struct BadOmen;

impl Hooks for BadOmen {}

pub fn hooks() -> Box<dyn Hooks> {
    Box::new(BadOmen)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_mod_id() {
        assert_eq!(MOD_ID, "badomen");
    }
}