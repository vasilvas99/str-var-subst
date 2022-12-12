use lazy_static::lazy_static;
use regex::{Captures, Regex};

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
///         return "John"
///     } else {
///         return "" // e.g. %{{no_var}} gets mapped to the empty string
///     }
/// });
/// assert_eq!(parsed_str, "Hi my name is John!");
/// println!("{}", parsed_str); // Hi my name is John!
/// ```
///
pub fn replace_variables<F>(template_text: &str, replacement_strategy: F) -> String
where
    F: Fn(&str) -> &str,
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

#[cfg(test)]
mod tests {
    static TEST_EXPR: &'static str = "This is a test string that has %{{test_num}} %{{test_num_2}}%{{test_num}} %{{test_num_2}} %{{empty_var}}variables";
    use crate::replace_variables;
    fn one_two_replace(variable: &str) -> &str {
        if variable == "test_num" {
            return "1";
        }
        if variable == "test_num_2" {
            return "2";
        }
        return "";
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
    fn test_json_template() {
        let in_template = include_str!("test_files/test_template.json.in");
        let expected_output = include_str!("test_files/test_output.json.in");
        let parsed = replace_variables(in_template, one_two_replace);
        println!("{}", parsed);
        assert_ne!(in_template, expected_output);
        assert_eq!(parsed, expected_output);
    }
}
