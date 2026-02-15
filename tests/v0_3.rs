// BREAKING CHANGE (v0.3): `Client` no longer implements `Eq` / `PartialEq`
// because it now holds a `reqwest::Client` internally.
#[test]
fn client_does_not_implement_eq() {
    use hatena_blog_api::Client;
    use static_assertions::assert_not_impl_any;
    assert_not_impl_any!(Client: Eq, PartialEq);
}

// BREAKING CHANGE (v0.3): `ParseEntry` has been renamed to `ParseEntryError`.
#[test]
fn parse_entry_error_is_exported() {
    use hatena_blog_api::ParseEntryError;
    let _error = ParseEntryError;
}

// BREAKING CHANGE (v0.3): `EntryId` no longer accepts empty strings.
#[test]
fn entry_id_rejects_empty_string() {
    use hatena_blog_api::EntryId;
    assert!("".parse::<EntryId>().is_err());
}
