//! # Evo
//! Evo is a graphical environment variable editor for Linux (library).

use std::collections::HashMap;

/// Fetch current environment variables
pub fn fetch_vars() -> HashMap<String, String> {
    let mut vars = HashMap::new();
    for (key, value) in std::env::vars_os() {
        vars.insert(
            key.to_str().unwrap().to_string(),
            value.to_str().unwrap().to_string(),
        );
    }
    vars
}

/// Set a new(s) variable(s)
pub fn set_var(_key: &str, _value: String) {
    todo!();
}

/// Edit an existing variable
pub fn edit_var(_key: &str) {
    todo!();
}

/// Unset an existing variable
pub fn unset_var(_key: &str) {
    todo!();
}

mod tests {
    #[test]
    fn test_fetch_vars() {
        let vars = crate::fetch_vars();
        assert_eq!(vars["SHLVL"], "0");
    }
}
