/// Implements `chimitheque_traits::searchable::Searchable` for a simple type.
///
/// The type must implement `Default` and have a `match_exact_search: bool`
/// field, an `Option<u64>` id field and a `String` text field. The id and text
/// field identifiers are also used as the database column names.
///
/// ```text
/// impl_searchable!(Tag, table: "tag", id: tag_id, text: tag_label);
/// ```
macro_rules! impl_searchable {
    ($type:ty, table: $table:literal, id: $id:ident, text: $text:ident) => {
        impl chimitheque_traits::searchable::Searchable for $type {
            fn create(&self) -> Self {
                Self::default()
            }

            fn set_exact_search(&mut self, match_exact_search: bool) {
                self.match_exact_search = match_exact_search;
            }

            fn get_exact_search(&self) -> bool {
                self.match_exact_search
            }

            fn get_table_name(&self) -> String {
                String::from($table)
            }

            fn get_id_field_name(&self) -> String {
                String::from(stringify!($id))
            }

            fn set_id_field(&mut self, id: u64) {
                self.$id = Some(id);
            }

            fn get_text_field_name(&self) -> String {
                String::from(stringify!($text))
            }

            fn set_text_field(&mut self, text: &str) {
                self.$text = text.to_string();
            }

            fn get_id(&self) -> Option<u64> {
                self.$id
            }

            fn get_text(&self) -> String {
                self.$text.clone()
            }
        }
    };
}

pub(crate) use impl_searchable;
