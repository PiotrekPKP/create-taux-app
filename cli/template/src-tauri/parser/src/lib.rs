// This macro attribute is used to tag types and/or structs that should not be parsed by the ts parser.
// You probably don't need to modify this.
#[proc_macro_attribute]
pub fn no_parse(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    input
}
