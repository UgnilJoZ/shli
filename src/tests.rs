use crate::parse::{split, EscapingState};

#[test]
fn parse_1() {
    let cmdline = " ";
    let state = EscapingState::process(cmdline);
    assert!(!state.single_quote);
    assert!(!state.double_quote);
    assert!(!state.backslash);
}

#[test]
fn parse_2() {
    let cmdline = "\"\"";
    let state = EscapingState::process(cmdline);
    assert!(!state.single_quote);
    assert!(!state.double_quote);
    assert!(!state.backslash);
}

#[test]
fn parse_3() {
    let cmdline = "\"";
    let state = EscapingState::process(cmdline);
    assert!(!state.single_quote);
    assert!(state.double_quote);
    assert!(!state.backslash);
}

#[test]
fn parse_4() {
    let cmdline = "\\\"";
    let state = EscapingState::process(cmdline);
    assert!(!state.single_quote);
    assert!(!state.double_quote);
    assert!(!state.backslash);
}

#[test]
fn split_1() {
    let cmdline = "\"A B C\"";
    let components = &split(cmdline);
    let normative_components = &["A B C".to_string()];
    assert_eq!(components, normative_components);
}

#[test]
fn split_2() {
    let cmdline = "\'A B C\'";
    let components = &split(cmdline);
    let normative_components = &["A B C".to_string()];
    assert_eq!(components, normative_components);
}
