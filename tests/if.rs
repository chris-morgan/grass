#![cfg(test)]

#[macro_use]
mod macros;

test!(
    if_toplevel_true,
    "@if true {\n  a {\n    color: foo;\n}\n}\n",
    "a {\n  color: foo;\n}\n"
);
test!(
    if_inner_true,
    "a {\n  @if true {\n    color: foo;\n}\n}\n",
    "a {\n  color: foo;\n}\n"
);
test!(
    if_toplevel_false,
    "@if false {\n  a {\n    color: foo;\n}\n}\n",
    ""
);
test!(
    if_inner_false,
    "a {\n  @if false {\n    color: foo;\n}\n}\n",
    ""
);
test!(
    if_else_toplevel_true,
    "@if true {\n  a {\n    color: foo;\n}\n} @else {\n  b {\n    color: bar;\n}\n}\n",
    "a {\n  color: foo;\n}\n"
);
test!(
    if_else_inner_true,
    "a {\n  @if true {\n    color: foo;\n} @else {\n    color: bar;\n}\n}\n",
    "a {\n  color: foo;\n}\n"
);
test!(
    if_else_toplevel_false,
    "@if false {\n  a {\n    color: foo;\n}\n} @else {\n  a {\n    color: bar;\n}\n}\n",
    "a {\n  color: bar;\n}\n"
);
test!(
    if_else_inner_false,
    "a {\n  @if false {\n    color: foo;\n} @else {\n    color: bar;\n}\n}\n",
    "a {\n  color: bar;\n}\n"
);
error!(
    no_brace_after_else,
    "@if false {} @else -}", "Error: expected \"{\"."
);
test!(
    if_else_if_no_else,
    "a {\n  @if false {\n    color: red;\n} @else if true {\n    color: blue;\n}\n}\n",
    "a {\n  color: blue;\n}\n"
);
test!(
    if_false_else_if_false_else,
    "a {\n  @if false {\n    color: red;\n} @else if false {\n    color: blue;\n} @else {\n    color: green;\n}\n}\n",
    "a {\n  color: green;\n}\n"
);
test!(
    if_false_else_if_true_else,
    "a {\n  @if false {\n    color: red;\n} @else if true {\n    color: blue;\n} @else {\n    color: green;\n}\n}\n",
    "a {\n  color: blue;\n}\n"
);
test!(
    if_inner_style_missing_semicolon,
    "a {\n  @if true {\n    color: red\n  }\n}\n",
    "a {\n  color: red;\n}\n"
);
test!(
    atrule_other_than_else_immediately_following,
    "a {\n  @if true {\n    b {\n      background: gray;\n    }\n  }\n\n  @if true {\n    b {\n      background: gray;\n    }\n  }\n}\n",
    "a b {\n  background: gray;\n}\na b {\n  background: gray;\n}\n"
);
test!(
    nested_if_in_function,
    "@function foo($value) {\n  @if true {\n    @if false {\n      @error foo;\n    }\n\n    @else {\n      @return $value;\n    }\n  }\n}
    a { color: foo(bar); }",
    "a {\n  color: bar;\n}\n"
);
test!(
    multiline_comments_surrounding_condition_empty,
    "@if/**/true/**/{ a { color: red; } }",
    "a {\n  color: red;\n}\n"
);
test!(
    multiline_comments_surrounding_condition,
    "@if/* pre 1 */true/* post 1 */{ a { color: red; } }",
    "a {\n  color: red;\n}\n"
);
test!(
    escaped_if,
    "@\\69 f true {\n  a {\n    color: red;\n  }\n}\n",
    "a {\n  color: red;\n}\n"
);
test!(
    uppercase_escaped_if,
    "@\\49 f true {\n  a {\n    color: red;\n  }\n}\n",
    "a {\n  color: red;\n}\n"
);
test!(
    escaped_else,
    "@if false {}\n\n@\\65lse {\n  a {\n    color: red;\n  }\n}\n",
    "a {\n  color: red;\n}\n"
);
test!(
    uppercase_escaped_else,
    "@if false {}\n\n@\\45lse {\n  a {\n    color: red;\n  }\n}\n",
    "a {\n  color: red;\n}\n"
);
error!(nothing_after_if, "@if", "Error: Expected expression.");
error!(
    nothing_after_dollar,
    "@if ${}", "Error: Expected identifier."
);
error!(no_condition, "@if{}", "Error: Expected expression.");
error!(
    nothing_after_open_curly,
    "@if foo {", "Error: expected \"}\"."
);
error!(
    condition_is_evaluated_eagerly,
    "@if 1 + 1 =s {\n}", "Error: expected \"=\"."
);
error!(
    nothing_after_escape,
    "@if \\", "Error: Expected expression."
);
error!(unclosed_dbl_quote, "@if true \" {}", "Error: Expected \".");
error!(unclosed_sgl_quote, "@if true ' {}", "Error: Expected '.");
error!(unclosed_call_args, "@if a({}", "Error: expected \")\".");
error!(nothing_after_div, "@if a/", "Error: Expected expression.");
