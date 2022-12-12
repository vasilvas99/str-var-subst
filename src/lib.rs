use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::env;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(%\{\{)([a-zA-Z_]\w*)(\}\})").unwrap();
}

/// Replaces variables in strings in the format %{{variable}}
/// Takes the template text as an input and a "replacement strategy" function
/// that provides the mapping between %{{variable}} and its value.
/// The delimiting character %, { and } are stripped before passing to the
/// mapping function
///
/// Example usage:
/// ```
/// use str_var_subst::replace_variables;
/// let test_str = "Hi my name is %{{name}}%{{no_var}}!";
/// let parsed_str = replace_variables(test_str, |var| {
///     if var == "name" {
///         return String::from("John")
///     } else {
///         return String::from("") // e.g. %{{no_var}} gets mapped to the empty string
///     }
/// });
/// assert_eq!(parsed_str, "Hi my name is John!");
/// println!("{}", parsed_str); // Hi my name is John!
/// ```
///
pub fn replace_variables<F>(template_text: &str, replacement_strategy: F) -> String
where
    F: Fn(&str) -> String
{
    let result = RE.replace_all(template_text, |caps: &Captures| {
        format!("{}", replacement_strategy(&remove_var_delimiters(&caps[0])))
    });

    String::from(result.to_string())
}

fn remove_var_delimiters(raw_variable: &str) -> String {
    raw_variable
        .replace("{", "")
        .replace("}", "")
        .replace("%", "")
        .trim()
        .to_owned()
}

/// Replace a variable in a string with its value from the environment
/// If the variable is unset it is replaced with "" (an empty string).
pub fn map_to_env(var: &str) -> String {
    match env::var(var) {
        Ok(val) => val,
        Err(_) => String::from("")
    }
}

#[cfg(test)]
mod tests {
    static TEST_EXPR: &'static str = "This is a test string that has %{{test_num}} %{{test_num_2}}%{{test_num}} %{{test_num_2}} %{{empty_var}}variables";
    use crate::*;
    fn one_two_replace(variable: &str) -> String {
        if variable == "test_num" {
            return String::from("1");
        }
        if variable == "test_num_2" {
            return String::from("2");
        }
        return String::from("");
    }
    #[test]
    fn test_simple_replacement() {
        let res = replace_variables(TEST_EXPR, one_two_replace);
        assert_eq!(
            res,
            String::from("This is a test string that has 1 21 2 variables")
        );
        println!("{}", res)
    }

    #[test]
    fn test_env_subst() {
        let key = "STR_VAR_SUBST_TEST_ENV_VAR";
        let val = "environment";
        let template = format!("This string uses a value from the %{{{{{}}}}}", key);
        env::set_var(key, val);
        let res = replace_variables(&template, map_to_env);
        env::remove_var(key);
        assert_eq!(res, "This string uses a value from the environment");
        println!("{}", res);
    }

    #[test]
    fn test_json_template() {
        let in_template = include_str!("test_files/test_template.json.in");
        let expected_output = include_str!("test_files/test_output.json.in");
        let parsed = replace_variables(in_template, one_two_replace);
        println!("{}", parsed);
        assert_ne!(in_template, expected_output);
        assert_eq!(parsed, expected_output);
    }
}
