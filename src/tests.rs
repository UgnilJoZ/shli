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
fn split_whitespace() {
    let cmdline = "A B C";
    let components = &split(cmdline);
    let normative_components = &["A".to_string(), "B".to_string(), "C".to_string()];
    assert_eq!(components, normative_components);
}

#[test]
fn split_dquote_escape() {
    let cmdline = "\"A B C\"";
    let components = &split(cmdline);
    let normative_components = &["A B C".to_string()];
    assert_eq!(components, normative_components);
}

#[test]
fn split_quote_escape() {
    let cmdline = "\'A B C\'";
    let components = &split(cmdline);
    let normative_components = &["A B C".to_string()];
    assert_eq!(components, normative_components);
}

#[test]
fn split_backslash_escape() {
    let cmdline = "A\\ B C";
    let components = &split(cmdline);
    let normative_components = &["A B".to_string(), "C".to_string()];
    assert_eq!(components, normative_components);
}

#[test]
fn split_alltogether() {
	let cmdline = "A \"\'\" B  \'\"\' \\\\ C";
	let components = &split(cmdline);
	let normative_components = &["A".to_string(), "\'".to_string(),
	"B".to_string(), "\"".to_string(), "\\".to_string(), "C".to_string()];
	assert_eq!(components, normative_components);
}