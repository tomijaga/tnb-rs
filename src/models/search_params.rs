use std::collections::HashMap;

/// Data storage type for query builders
pub type SearchParams<'a> = HashMap<&'a str, String>;

/// Trait for endpoints that return paginated data
pub trait PaginatedSearchParams<'a> {
    /// Get a reference to the data store of
    fn get_mut_params(&mut self) -> &mut SearchParams<'a>;

    /// The number of items to retrieve
    /// - The min is 1
    /// - The max is 100
    ///
    fn limit(&mut self, value: u8) -> &mut Self {
        let v = value.to_string();
        self.get_mut_params().insert("limit", v);
        self
    }

    /// The number of items from the start of the data to skip
    fn offset(&mut self, value: u8) -> &mut Self {
        let v = value.to_string();

        self.get_mut_params().insert("offset", v);
        self
    }
}
