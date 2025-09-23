#[test_case::test_case(
    "macro_name_string_literal.rs"; "macro_name must be Ident, not string literal"
)]
#[test_case::test_case(
    "handle_name_string_literal.rs"; "handle_name must be Ident, not string literal"
)]
#[test_case::test_case(
    "missing_semicolon_before_opts.rs"; "missing semicolon before ExtraParams"
)]
#[test_case::test_case(
    "missing_equals_in_opt.rs"; "missing equals in ExtraParams assignment"
)]
#[test_case::test_case(
    "missing_comma_between_opts.rs"; "missing comma between ExtraParams"
)]
#[test_case::test_case(
    "handle_name_invalid_ident.rs"; "handle_name value must be a valid identifier"
)]
#[test_case::test_case(
    "macro_name_keyword_ident.rs"; "macro_name cannot be a reserved keyword"
)]
#[test_case::test_case(
    "is_not_dyn_compat.rs"; "trait is not dyn compatible"
)]
fn test_failures(path: &'static str) {
    let t = trybuild::TestCases::new();

    t.compile_fail(format!("tests/failures/{path}"));
}
