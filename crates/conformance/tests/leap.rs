//! The `leap` suite: one test per scenario in `features/suites/leap.feature`.
//!
//! The proof is checked by the Lean kernel when `just vv` runs
//! `prismpm verify` on `app/`. What this test holds fixed is the claim the
//! proof is about: the theorem's exact statement, the Gregorian rule in the
//! proposition it decides, and that the answer the application returns is
//! decided by the function it proves. A weakened rule, a dropped condition or
//! an answer decided some other way would still verify, and would no longer
//! be the claim `LY-01` makes.

use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/conformance is two below the root")
        .to_path_buf()
}

/// The body of the top-level declaration named `name`, up to its closing brace.
fn declaration<'a>(model: &'a str, name: &str) -> &'a str {
    let start = model
        .find(&format!("\n  name: {name},\n"))
        .unwrap_or_else(|| panic!("{name} is not declared"));
    let end = model[start..]
        .find("\n}\n")
        .unwrap_or_else(|| panic!("{name} is not closed"));
    &model[start..start + end]
}

/// `LY-01`: The app answers leap exactly when the year is divisible by 4 and, if divisible by 100, also by 400
#[test]
fn leap_ly_01() {
    // The theorem lives in its own module, which the executable app does not
    // import: its proof needs `LexLeanRuntime.Laws`, and only the app's import
    // closure is compiled into the guest.
    let model = std::fs::read_to_string(root().join("app/src/LeapYear.lex.tex"))
        .expect("the leap-year model");
    let rule = std::fs::read_to_string(root().join("app/src/LeapRule.lex.tex"))
        .expect("the leap-year rule");
    let lexlean =
        std::fs::read_to_string(root().join("app/lexlean.toml")).expect("the LexLean project");
    assert!(
        lexlean.contains("\"src/LeapRule.lex.tex\""),
        "the rule module must be an entrypoint, or its theorem is never checked"
    );

    let decides = declaration(&rule, "isLeap_decides");
    assert!(
        decides.contains("statement: (((LeapYear::isLeap year) = true) <-> (Leap year)),"),
        "LY-01's statement changed: isLeap must be true exactly when Leap holds:\n{decides}"
    );
    assert!(
        decides.contains("parameters: [year : Nat],"),
        "LY-01 quantifies over every natural year:\n{decides}"
    );

    // The proposition is the Gregorian rule: divisible by 4, and if divisible
    // by 100 then also by 400.
    let leap = declaration(&rule, "Leap");
    assert!(
        leap.contains(
            "body: (((prim remainder year 4 0 : Nat) = 0) /\\ (((prim remainder year 100 0 : Nat) = 0) -> ((prim remainder year 400 0 : Nat) = 0)))"
        ),
        "Leap must state the Gregorian rule:\n{leap}"
    );

    let answer = declaration(&model, "answer");
    assert!(
        answer.contains("(if (isLeap (prim to_nat year : Nat)) then"),
        "the answer must be decided by isLeap, the function LY-01 proves:\n{answer}"
    );
    let dispatch = declaration(&model, "dispatch");
    assert!(
        dispatch.contains("| Option.some year => (answer year)"),
        "every parsed year must be answered by answer:\n{dispatch}"
    );
}
