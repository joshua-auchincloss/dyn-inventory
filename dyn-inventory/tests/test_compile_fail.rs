#[test_case::test_case("init_name_invalid_ident.rs"; "invalid ident in struct position")]
#[test_case::test_case("init_name_keyword_ident.rs"; "keyword used as macro_name value")]
#[test_case::test_case("init_name_string_literal.rs"; "string literal where ident expected for macro_name")]
#[test_case::test_case("is_not_dyn_compat.rs"; "trait not dyn compatible")]
#[test_case::test_case("missing_comma_between_opts.rs"; "missing comma between extraparams")]
#[test_case::test_case("missing_equals_in_opt.rs"; "missing equals in extraparams entry")]
#[test_case::test_case("missing_semicolon_before_opts.rs"; "missing semicolon before extraparams list")]
#[test_case::test_case("test_basic_fail.rs"; "unknown extraparams keyword")]
fn test_failures(path: &'static str) {
    let t = trybuild::TestCases::new();
    t.compile_fail(format!("tests/failures/{path}"));
}
