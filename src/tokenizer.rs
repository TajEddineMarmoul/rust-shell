use std::vec;

pub fn tokenize(input: &str) -> Vec<String> {
    let input_trimmed = input.trim();

    let mut is_whitespace = false;
    let mut is_double_quote = false;
    let mut is_single_quote = false;

    let mut final_tokens = vec![];
    let mut next_token = String::new();

    for char in input_trimmed.chars() {
        if char.is_whitespace() {
            if !is_double_quote && !is_single_quote {
                if !is_whitespace {
                    is_whitespace = true;
                    final_tokens.push(next_token);
                    next_token = String::new();
                }
                continue;
            }
        } else if char == '"' && !is_single_quote {
            if is_double_quote {
                final_tokens.push(next_token);
                next_token = String::new();
                is_whitespace = true
            }
            is_double_quote = !is_double_quote;
            continue;
        } else if char == '\'' && !is_double_quote {
            if is_single_quote {
                final_tokens.push(next_token);
                next_token = String::new();
                is_whitespace = true
            }
            is_single_quote = !is_single_quote;
            continue;
        } else {
            is_whitespace = false;
        }

        next_token += &char.clone().to_string();
    }
    if !next_token.is_empty() {
        final_tokens.push(next_token);
    }
    final_tokens
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::tokenizer::tokenize;

    #[test]
    fn splits_on_spaces() {
        assert_eq!(tokenize("ls  -a"), vec!["ls", "-a"])
    }

    #[test]
    fn splits_on_t_tab() {
        assert_eq!(tokenize("ls\t-a"), vec!["ls", "-a"])
    }

    #[test]
    fn split_with_double_quote() {
        assert_eq!(
            tokenize(r#"echo "Hello World!" foo "#),
            vec!["echo", "Hello World!", "foo"]
        )
    }
    #[test]
    fn split_with_single_quote() {
        assert_eq!(
            tokenize(r#"echo 'say' 'Hello World!' "#),
            vec!["echo", "say", "Hello World!"]
        )
    }
    #[test]
    fn split_with_single_inside_double_quote() {
        assert_eq!(
            tokenize(r#"echo "'say' 'Hello World!'" "#),
            vec!["echo", "'say' 'Hello World!'"]
        )
    }
    #[test]
    fn split_with_double_inside_single_quote() {
        assert_eq!(
            tokenize(r#"echo '"say" "Hello World!"' "#),
            vec!["echo", r#""say" "Hello World!""#]
        )
    }
    #[test]
    fn quote_mid_word() {
        assert_eq!(tokenize(r#"foo"bar baz"qux"#), vec!["foobarbazqux"])
    }

    #[test]
    fn adjacent_quotes() {
        assert_eq!(tokenize(r#"echo "a""b""#), vec!["echo", "ab"])
    }
}
