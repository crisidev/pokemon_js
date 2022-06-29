// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetPokemonSpeciesOutput {
    /// The name for this resource.
    pub name: std::string::String,
    /// A list of flavor text entries for this Pokémon species.
    pub flavor_text_entries: std::vec::Vec<crate::model::FlavorText>,
}
impl GetPokemonSpeciesOutput {
    /// The name for this resource.
    pub fn name(&self) -> &str {
        use std::ops::Deref;
        self.name.deref()
    }
    /// A list of flavor text entries for this Pokémon species.
    pub fn flavor_text_entries(&self) -> &[crate::model::FlavorText] {
        use std::ops::Deref;
        self.flavor_text_entries.deref()
    }
}
impl std::fmt::Debug for GetPokemonSpeciesOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetPokemonSpeciesOutput");
        formatter.field("name", &self.name);
        formatter.field("flavor_text_entries", &self.flavor_text_entries);
        formatter.finish()
    }
}
/// See [`GetPokemonSpeciesOutput`](crate::output::GetPokemonSpeciesOutput).
pub mod get_pokemon_species_output {

    /// A builder for [`GetPokemonSpeciesOutput`](crate::output::GetPokemonSpeciesOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) flavor_text_entries:
            std::option::Option<std::vec::Vec<crate::model::FlavorText>>,
    }
    impl Builder {
        /// The name for this resource.
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// The name for this resource.
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// Appends an item to `flavor_text_entries`.
        ///
        /// To override the contents of this collection use [`set_flavor_text_entries`](Self::set_flavor_text_entries).
        ///
        /// A list of flavor text entries for this Pokémon species.
        pub fn flavor_text_entries(mut self, input: crate::model::FlavorText) -> Self {
            let mut v = self.flavor_text_entries.unwrap_or_default();
            v.push(input);
            self.flavor_text_entries = Some(v);
            self
        }
        /// A list of flavor text entries for this Pokémon species.
        pub fn set_flavor_text_entries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::FlavorText>>,
        ) -> Self {
            self.flavor_text_entries = input;
            self
        }
        /// Consumes the builder and constructs a [`GetPokemonSpeciesOutput`](crate::output::GetPokemonSpeciesOutput).
        pub fn build(
            self,
        ) -> Result<crate::output::GetPokemonSpeciesOutput, aws_smithy_http::operation::BuildError>
        {
            Ok(
                crate::output::GetPokemonSpeciesOutput {
                    name: self.name
                        .unwrap_or_default()
                    ,
                    flavor_text_entries: self.flavor_text_entries
                        .ok_or(
                            aws_smithy_http::operation::BuildError::MissingField { field: "flavor_text_entries", details: "flavor_text_entries was not specified but it is required when building GetPokemonSpeciesOutput" }
                        )?
                    ,
                }
            )
        }
    }
}
impl GetPokemonSpeciesOutput {
    /// Creates a new builder-style object to manufacture [`GetPokemonSpeciesOutput`](crate::output::GetPokemonSpeciesOutput).
    pub fn builder() -> crate::output::get_pokemon_species_output::Builder {
        crate::output::get_pokemon_species_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct EmptyOperationOutput {}
impl std::fmt::Debug for EmptyOperationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("EmptyOperationOutput");
        formatter.finish()
    }
}
/// See [`EmptyOperationOutput`](crate::output::EmptyOperationOutput).
pub mod empty_operation_output {

    /// A builder for [`EmptyOperationOutput`](crate::output::EmptyOperationOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`EmptyOperationOutput`](crate::output::EmptyOperationOutput).
        pub fn build(self) -> crate::output::EmptyOperationOutput {
            crate::output::EmptyOperationOutput {}
        }
    }
}
impl EmptyOperationOutput {
    /// Creates a new builder-style object to manufacture [`EmptyOperationOutput`](crate::output::EmptyOperationOutput).
    pub fn builder() -> crate::output::empty_operation_output::Builder {
        crate::output::empty_operation_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetServerStatisticsOutput {
    /// The number of calls executed by the server.
    pub calls_count: i64,
}
impl GetServerStatisticsOutput {
    /// The number of calls executed by the server.
    pub fn calls_count(&self) -> i64 {
        self.calls_count
    }
}
impl std::fmt::Debug for GetServerStatisticsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetServerStatisticsOutput");
        formatter.field("calls_count", &self.calls_count);
        formatter.finish()
    }
}
/// See [`GetServerStatisticsOutput`](crate::output::GetServerStatisticsOutput).
pub mod get_server_statistics_output {

    /// A builder for [`GetServerStatisticsOutput`](crate::output::GetServerStatisticsOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) calls_count: std::option::Option<i64>,
    }
    impl Builder {
        /// The number of calls executed by the server.
        pub fn calls_count(mut self, input: i64) -> Self {
            self.calls_count = Some(input);
            self
        }
        /// The number of calls executed by the server.
        pub fn set_calls_count(mut self, input: std::option::Option<i64>) -> Self {
            self.calls_count = input;
            self
        }
        /// Consumes the builder and constructs a [`GetServerStatisticsOutput`](crate::output::GetServerStatisticsOutput).
        pub fn build(self) -> crate::output::GetServerStatisticsOutput {
            crate::output::GetServerStatisticsOutput {
                calls_count: self.calls_count.unwrap_or_default(),
            }
        }
    }
}
impl GetServerStatisticsOutput {
    /// Creates a new builder-style object to manufacture [`GetServerStatisticsOutput`](crate::output::GetServerStatisticsOutput).
    pub fn builder() -> crate::output::get_server_statistics_output::Builder {
        crate::output::get_server_statistics_output::Builder::default()
    }
}
