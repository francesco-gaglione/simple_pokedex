use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PokemonSpeciesResponseDto {
    pub id: i32,
    pub name: String,
    pub order: i32,
    pub gender_rate: i32,
    pub capture_rate: i32,
    pub base_happiness: i32,
    pub is_baby: bool,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub hatch_counter: i32,
    pub has_gender_differences: bool,
    pub forms_switchable: bool,
    pub growth_rate: NamedApiResource,
    pub pokedex_numbers: Vec<PokemonDexEntry>,
    pub egg_groups: Vec<NamedApiResource>,
    pub color: NamedApiResource,
    pub shape: NamedApiResource,
    pub evolves_from_species: Option<NamedApiResource>,
    pub evolution_chain: ApiResource,
    pub habitat: Option<NamedApiResource>,
    pub generation: NamedApiResource,
    pub names: Vec<Name>,
    pub flavor_text_entries: Vec<FlavorText>,
    pub form_descriptions: Vec<Description>,
    pub genera: Vec<Genus>,
    pub varieties: Vec<PokemonVariety>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NamedApiResource {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApiResource {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PokemonDexEntry {
    pub entry_number: i32,
    pub pokedex: NamedApiResource,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Name {
    pub name: String,
    pub language: NamedApiResource,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FlavorText {
    #[serde(rename = "flavor_text")]
    pub text: String,
    pub language: NamedApiResource,
    pub version: NamedApiResource,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Description {
    pub description: String,
    pub language: NamedApiResource,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Genus {
    pub genus: String,
    pub language: NamedApiResource,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PokemonVariety {
    pub is_default: bool,
    pub pokemon: NamedApiResource,
}

impl PokemonSpeciesResponseDto {
    pub fn get_english_description(&self) -> Option<String> {
        self.flavor_text_entries
            .iter()
            .find(|entry| entry.language.name == "en")
            .map(|entry| {
                entry
                    .text
                    .replace("\\n", " ")
                    .replace("\n", " ")
                    .replace("\u{0c}", " ")
                    .replace("\\f", " ")
                    .replace("\r", " ")
                    .trim()
                    .to_string()
            })
    }

    pub fn get_habitat(&self) -> Option<String> {
        self.habitat.as_ref().map(|habitat| {
            habitat
                .name
                .replace("-", " ")
                .split_whitespace()
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_english_description_extracts_correct_language() {
        let dto = PokemonSpeciesResponseDto {
            id: 1,
            name: "test".to_string(),
            order: 1,
            gender_rate: 0,
            capture_rate: 0,
            base_happiness: 0,
            is_baby: false,
            is_legendary: false,
            is_mythical: false,
            hatch_counter: 0,
            has_gender_differences: false,
            forms_switchable: false,
            growth_rate: NamedApiResource {
                name: "slow".to_string(),
                url: "https://pokeapi.co/api/v2/growth-rate/1/".to_string(),
            },
            pokedex_numbers: vec![],
            egg_groups: vec![],
            color: NamedApiResource {
                name: "red".to_string(),
                url: "".to_string(),
            },
            shape: NamedApiResource {
                name: "blob".to_string(),
                url: "".to_string(),
            },
            evolves_from_species: None,
            evolution_chain: ApiResource {
                url: "".to_string(),
            },
            habitat: None,
            generation: NamedApiResource {
                name: "generation-i".to_string(),
                url: "".to_string(),
            },
            names: vec![],
            flavor_text_entries: vec![
                FlavorText {
                    text: "Italian\\ntext".to_string(),
                    language: NamedApiResource {
                        name: "it".to_string(),
                        url: "".to_string(),
                    },
                    version: NamedApiResource {
                        name: "".to_string(),
                        url: "".to_string(),
                    },
                },
                FlavorText {
                    text: r"English\ntext\fwith\nspecial chars".to_string(),
                    language: NamedApiResource {
                        name: "en".to_string(),
                        url: "".to_string(),
                    },
                    version: NamedApiResource {
                        name: "".to_string(),
                        url: "".to_string(),
                    },
                },
            ],
            form_descriptions: vec![],
            genera: vec![],
            varieties: vec![],
        };

        let result = dto.get_english_description();

        assert_eq!(result, Some("English text with special chars".to_string()));
    }

    #[test]
    fn test_get_habitat_capitalizes_correctly() {
        let dto = PokemonSpeciesResponseDto {
            id: 1,
            name: "test".to_string(),
            order: 1,
            gender_rate: 0,
            capture_rate: 0,
            base_happiness: 0,
            is_baby: false,
            is_legendary: false,
            is_mythical: false,
            hatch_counter: 0,
            has_gender_differences: false,
            forms_switchable: false,
            growth_rate: NamedApiResource {
                name: "slow".to_string(),
                url: "https://pokeapi.co/api/v2/growth-rate/1/".to_string(),
            },
            pokedex_numbers: vec![],
            egg_groups: vec![],
            color: NamedApiResource {
                name: "red".to_string(),
                url: "".to_string(),
            },
            shape: NamedApiResource {
                name: "blob".to_string(),
                url: "".to_string(),
            },
            evolves_from_species: None,
            evolution_chain: ApiResource {
                url: "".to_string(),
            },
            habitat: Some(NamedApiResource {
                name: "rare-cave".to_string(),
                url: "".to_string(),
            }),
            generation: NamedApiResource {
                name: "generation-i".to_string(),
                url: "".to_string(),
            },
            names: vec![],
            flavor_text_entries: vec![],
            form_descriptions: vec![],
            genera: vec![],
            varieties: vec![],
        };

        let result = dto.get_habitat();

        assert_eq!(result, Some("Rare Cave".to_string()));
    }

    #[test]
    fn test_get_habitat_returns_none_when_missing() {
        let dto = PokemonSpeciesResponseDto {
            id: 1,
            name: "test".to_string(),
            order: 1,
            gender_rate: 0,
            capture_rate: 0,
            base_happiness: 0,
            is_baby: false,
            is_legendary: false,
            is_mythical: false,
            hatch_counter: 0,
            has_gender_differences: false,
            forms_switchable: false,
            growth_rate: NamedApiResource {
                name: "slow".to_string(),
                url: "https://pokeapi.co/api/v2/growth-rate/1/".to_string(),
            },
            pokedex_numbers: vec![],
            egg_groups: vec![],
            color: NamedApiResource {
                name: "red".to_string(),
                url: "".to_string(),
            },
            shape: NamedApiResource {
                name: "blob".to_string(),
                url: "".to_string(),
            },
            evolves_from_species: None,
            evolution_chain: ApiResource {
                url: "".to_string(),
            },
            habitat: None,
            generation: NamedApiResource {
                name: "generation-i".to_string(),
                url: "".to_string(),
            },
            names: vec![],
            flavor_text_entries: vec![],
            form_descriptions: vec![],
            genera: vec![],
            varieties: vec![],
        };

        assert_eq!(dto.get_habitat(), None);
    }

    #[test]
    fn test_get_english_description_returns_none_when_no_english() {
        let dto = PokemonSpeciesResponseDto {
            id: 1,
            name: "test".to_string(),
            order: 1,
            gender_rate: 0,
            capture_rate: 0,
            base_happiness: 0,
            is_baby: false,
            is_legendary: false,
            is_mythical: false,
            hatch_counter: 0,
            has_gender_differences: false,
            forms_switchable: false,
            growth_rate: NamedApiResource {
                name: "slow".to_string(),
                url: "https://pokeapi.co/api/v2/growth-rate/1/".to_string(),
            },
            pokedex_numbers: vec![],
            egg_groups: vec![],
            color: NamedApiResource {
                name: "red".to_string(),
                url: "".to_string(),
            },
            shape: NamedApiResource {
                name: "blob".to_string(),
                url: "".to_string(),
            },
            evolves_from_species: None,
            evolution_chain: ApiResource {
                url: "".to_string(),
            },
            habitat: None,
            generation: NamedApiResource {
                name: "generation-i".to_string(),
                url: "".to_string(),
            },
            names: vec![],
            flavor_text_entries: vec![FlavorText {
                text: "Italian\\ntext".to_string(),
                language: NamedApiResource {
                    name: "it".to_string(),
                    url: "".to_string(),
                },
                version: NamedApiResource {
                    name: "".to_string(),
                    url: "".to_string(),
                },
            }],
            form_descriptions: vec![],
            genera: vec![],
            varieties: vec![],
        };

        let result = dto.get_english_description();

        assert_eq!(result, None);
    }

    #[test]
    fn test_get_english_description_handles_empty_text() {
        let dto = PokemonSpeciesResponseDto {
            id: 1,
            name: "test".to_string(),
            order: 1,
            gender_rate: 0,
            capture_rate: 0,
            base_happiness: 0,
            is_baby: false,
            is_legendary: false,
            is_mythical: false,
            hatch_counter: 0,
            has_gender_differences: false,
            forms_switchable: false,
            growth_rate: NamedApiResource {
                name: "slow".to_string(),
                url: "https://pokeapi.co/api/v2/growth-rate/1/".to_string(),
            },
            pokedex_numbers: vec![],
            egg_groups: vec![],
            color: NamedApiResource {
                name: "red".to_string(),
                url: "".to_string(),
            },
            shape: NamedApiResource {
                name: "blob".to_string(),
                url: "".to_string(),
            },
            evolves_from_species: None,
            evolution_chain: ApiResource {
                url: "".to_string(),
            },
            habitat: None,
            generation: NamedApiResource {
                name: "generation-i".to_string(),
                url: "".to_string(),
            },
            names: vec![],
            flavor_text_entries: vec![FlavorText {
                text: "".to_string(),
                language: NamedApiResource {
                    name: "en".to_string(),
                    url: "".to_string(),
                },
                version: NamedApiResource {
                    name: "".to_string(),
                    url: "".to_string(),
                },
            }],
            form_descriptions: vec![],
            genera: vec![],
            varieties: vec![],
        };

        let result = dto.get_english_description();

        assert_eq!(result, Some("".to_string()));
    }

    #[test]
    fn test_get_habitat_handles_single_word() {
        let dto = PokemonSpeciesResponseDto {
            id: 1,
            name: "test".to_string(),
            order: 1,
            gender_rate: 0,
            capture_rate: 0,
            base_happiness: 0,
            is_baby: false,
            is_legendary: false,
            is_mythical: false,
            hatch_counter: 0,
            has_gender_differences: false,
            forms_switchable: false,
            growth_rate: NamedApiResource {
                name: "slow".to_string(),
                url: "https://pokeapi.co/api/v2/growth-rate/1/".to_string(),
            },
            pokedex_numbers: vec![],
            egg_groups: vec![],
            color: NamedApiResource {
                name: "red".to_string(),
                url: "".to_string(),
            },
            shape: NamedApiResource {
                name: "blob".to_string(),
                url: "".to_string(),
            },
            evolves_from_species: None,
            evolution_chain: ApiResource {
                url: "".to_string(),
            },
            habitat: Some(NamedApiResource {
                name: "forest".to_string(),
                url: "".to_string(),
            }),
            generation: NamedApiResource {
                name: "generation-i".to_string(),
                url: "".to_string(),
            },
            names: vec![],
            flavor_text_entries: vec![],
            form_descriptions: vec![],
            genera: vec![],
            varieties: vec![],
        };

        let result = dto.get_habitat();

        assert_eq!(result, Some("Forest".to_string()));
    }
}
