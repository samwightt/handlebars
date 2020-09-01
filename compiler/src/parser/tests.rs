use super::*;

#[test]
fn html_char_parses_single_character() {
    assert_eq!(html_char("a").unwrap(), ("", 'a'));
    assert_eq!(html_char(" ").unwrap(), ("", ' '));
}

#[test]
fn html_char_parses_single_character_in_string() {
    assert_eq!(html_char("abcd").unwrap(), ("bcd", 'a'));
    assert_eq!(html_char("bcd").unwrap(), ("cd", 'b'));
    assert_eq!(html_char("cd").unwrap(), ("d", 'c'));
}

#[test]
fn html_char_fails_on_bad_characters() {
    html_char("{").unwrap_err();
    html_char("}").unwrap_err();
    html_char("<").unwrap_err();
    html_char(">").unwrap_err();
}

#[test]
fn html_char_fails_on_empty_characters() {
    html_char("").unwrap_err();
}

#[test]
fn html_text_accepts_valid_input() {
    assert_eq!(
        html_text("abcdefg").unwrap(),
        ("", HTMLChild::Text("abcdefg".to_string()))
    );
    assert_eq!(
        html_text("abc def ghi - - i").unwrap(),
        ("", HTMLChild::Text("abc def ghi - - i".to_string()))
    );
    assert_eq!(
        html_text("123456789$#%&^(").unwrap(),
        ("", HTMLChild::Text("123456789$#%&^(".to_string()))
    );
    assert_eq!(
        html_text("    ").unwrap(),
        ("", HTMLChild::Text("    ".to_string()))
    );
}

#[test]
fn html_text_fails_on_invalid_input() {
    html_text("<>{}{}{}<>").unwrap_err();
    html_text("<").unwrap_err();
    html_text("<asdfkljfsdf 345987324").unwrap_err();
}

#[test]
fn html_text_fails_on_empty_string() {
    html_text("").unwrap_err();
}

#[test]
fn tag_name_parses_valid_input() {
    assert_eq!(tag_name("asdf1234").unwrap(), ("", "asdf1234"));
    assert_eq!(tag_name("ASDFGHJKL").unwrap(), ("", "ASDFGHJKL"));
    assert_eq!(tag_name("ASDFGHJKL!").unwrap(), ("!", "ASDFGHJKL"));
}

#[test]
fn tag_name_fails_on_invalid_input() {
    tag_name(" ").unwrap_err();
    tag_name(" asdfadf").unwrap_err();
    tag_name("!adsfadsf").unwrap_err();
    tag_name("#$#%#%#").unwrap_err();
}

#[test]
fn tag_name_fails_on_empty_input() {
    tag_name("").unwrap_err();
}

#[test]
fn attribute_char_parses_valid_input() {
    assert_eq!(attribute_char("asdfasdf").unwrap(), ("sdfasdf", 'a'));
    assert_eq!(attribute_char("f").unwrap(), ("", 'f'));
    assert_eq!(attribute_char("1").unwrap(), ("", '1'));
    assert_eq!(attribute_char("a ").unwrap(), (" ", 'a'));
    assert_eq!(attribute_char("$").unwrap(), ("", '$'));
}

#[test]
fn attribute_char_fails_on_invalid_input() {
    attribute_char(" ").unwrap_err();
    attribute_char("\"").unwrap_err();
    attribute_char("<").unwrap_err();
    attribute_char(">").unwrap_err();
    attribute_char("'").unwrap_err();
    attribute_char(" ").unwrap_err();
    attribute_char("\t").unwrap_err();
    attribute_char("\n").unwrap_err();
    attribute_char("\r").unwrap_err();
    attribute_char(" asdfadf").unwrap_err();
}

#[test]
fn attribute_char_fails_on_empty_string() {
    attribute_char("").unwrap_err();
}

#[test]
fn attr_val_doub_str_valid_parse() {
    assert_eq!(
        attribute_value_double_string("\"asdfwre$%\"").unwrap(),
        ("", HTMLValue::String("asdfwre$%".to_string()))
    );
    assert_eq!(
        attribute_value_double_string("\"1234567890!@#$%^&*()-+{}\\/?<>:\"").unwrap(),
        (
            "",
            HTMLValue::String("1234567890!@#$%^&*()-+{}\\/?<>:".to_string())
        )
    );
    assert_eq!(
        attribute_value_double_string("\"''''\"").unwrap(),
        ("", HTMLValue::String("''''".to_string()))
    );
    assert_eq!(
        attribute_value_double_string("\"    \"").unwrap(),
        ("", HTMLValue::String("    ".to_string()))
    );
}

#[test]
fn attr_val_doub_str_invalid_input_fails() {
    attribute_value_double_string("  ").unwrap_err();
    attribute_value_double_string("\"asdfasdfaf").unwrap_err();
    attribute_value_double_string("'asdfadf'").unwrap_err();
    attribute_value_double_string(" \"asdfasdf\"").unwrap_err();
    attribute_value_double_string("").unwrap_err();
}

#[test]
fn attr_val_sing_str_valid_parse() {
    assert_eq!(
        attribute_value_single_string("'asdfwre$%'").unwrap(),
        ("", HTMLValue::String("asdfwre$%".to_string()))
    );
    assert_eq!(
        attribute_value_single_string("'1234567890!@#$%^&*()-+{}\\/?<>:'").unwrap(),
        (
            "",
            HTMLValue::String("1234567890!@#$%^&*()-+{}\\/?<>:".to_string())
        )
    );
    assert_eq!(
        attribute_value_single_string("'\"'").unwrap(),
        ("", HTMLValue::String("\"".to_string()))
    );
    assert_eq!(
        attribute_value_single_string("'    '").unwrap(),
        ("", HTMLValue::String("    ".to_string()))
    );
}

#[test]
fn attr_val_sing_str_invalid_input_fails() {
    attribute_value_single_string("  ").unwrap_err();
    attribute_value_single_string("'asdfasdfaf").unwrap_err();
    attribute_value_single_string("\"asdfadf\"").unwrap_err();
    attribute_value_single_string(" 'asdfasdf'").unwrap_err();
    attribute_value_single_string("").unwrap_err();
}

#[test]
fn attribute_value_parses_double_string() {
    assert_eq!(
        attribute_value("\"asdfasdf\"").unwrap(),
        ("", HTMLValue::String("asdfasdf".to_string()))
    );
}

#[test]
fn attribute_value_parses_single_string() {
    assert_eq!(
        attribute_value("'asdfasdf'").unwrap(),
        ("", HTMLValue::String("asdfasdf".to_string()))
    );
}

#[test]
fn attribute_value_invalid_input_fails() {
    attribute_value("asdfadsf").unwrap_err();
    attribute_value("").unwrap_err();
    attribute_value("asdfkkj3223489234234 ").unwrap_err();
    attribute_value("\"asdf").unwrap_err();
    attribute_value("'asdfasdf").unwrap_err();
}

#[test]
fn attribute_name_parses_valid_name() {
    assert_eq!(
        attribute_name("asdfasdf-asdfasdf").unwrap(),
        ("", "asdfasdf-asdfasdf".to_string())
    );
    assert_eq!(attribute_name("12345").unwrap(), ("", "12345".to_string()));
    assert_eq!(
        attribute_name("asdfasdf <").unwrap(),
        (" <", "asdfasdf".to_string())
    );
    assert_eq!(
        attribute_name("!@#$%^&*()_+").unwrap(),
        ("", "!@#$%^&*()_+".to_string())
    );
}

#[test]
fn attribute_name_fails_invalid_input() {
    attribute_name(" ").unwrap_err();
    attribute_name("").unwrap_err();
    attribute_name(" asdfasdf").unwrap_err();
    attribute_name("\"asdfadf").unwrap_err();
    attribute_name("<asdf>").unwrap_err();
}
