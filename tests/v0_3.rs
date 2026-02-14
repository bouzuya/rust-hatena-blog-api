// BREAKING CHANGE (v0.3): `Client` no longer implements `Eq` / `PartialEq`
// because it now holds a `reqwest::Client` internally.
#[test]
fn client_does_not_implement_eq() {
    use hatena_blog_api::Client;
    use static_assertions::assert_not_impl_any;
    assert_not_impl_any!(Client: Eq, PartialEq);
}
