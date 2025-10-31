#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pokemon {
    name: String,
    description: String,
    habitat: String,
    is_legendary: bool,
}

impl Pokemon {
    pub fn new(name: String, description: String, habitat: String, is_legendary: bool) -> Self {
        Self {
            name,
            description,
            habitat,
            is_legendary,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn habitat(&self) -> &str {
        &self.habitat
    }

    pub fn is_legendary(&self) -> bool {
        self.is_legendary
    }

    pub fn is_cave(&self) -> bool {
        self.habitat == "cave"
    }

    pub fn set_translated_description(&mut self, new_description: String) {
        self.description = new_description;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pokemon_creation() {
        let pokemon = Pokemon::new(
            "Pikachu".to_string(),
            "Electric mouse".to_string(),
            "cave".to_string(),
            false,
        );

        assert_eq!(pokemon.name(), "Pikachu");
        assert_eq!(pokemon.habitat(), "cave");
        assert!(!pokemon.is_legendary());
    }

    #[test]
    fn test_is_cave_detection() {
        let cave_pokemon = Pokemon::new(
            "Onix".to_string(),
            "Rock snake".to_string(),
            "cave".to_string(),
            false,
        );

        assert!(cave_pokemon.is_cave());
    }

    #[test]
    fn test_set_translated_description() {
        let mut pokemon = Pokemon::new(
            "Charmander".to_string(),
            "Fire lizard".to_string(),
            "grassland".to_string(),
            false,
        );

        pokemon.set_translated_description("Verily, a flame-born creature".to_string());

        assert_eq!(pokemon.description(), "Verily, a flame-born creature");
    }

    #[test]
    fn test_legendary_pokemon() {
        let legendary = Pokemon::new(
            "Mewtwo".to_string(),
            "Genetic clone".to_string(),
            "laboratory".to_string(),
            true,
        );

        assert!(legendary.is_legendary());
    }
}
