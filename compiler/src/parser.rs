use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, multispace0, none_of},
    multi::{many0, many1},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum HTMLChild {
    Text(String),
    Element(Box<HTMLElement>),
}

#[derive(Debug, PartialEq)]
pub enum HTMLStartTag {
    Tag(String, Vec<HTMLAttribute>),
}

#[derive(Debug, PartialEq)]
pub enum HTMLEndTag {
    Tag(String),
}

#[derive(Debug, PartialEq)]
pub enum HTMLValue {
    String(String),
}

#[derive(Debug, PartialEq)]
pub enum HTMLAttribute {
    Attribute { name: String, value: HTMLValue },
}

#[derive(Debug, PartialEq)]
pub enum HTMLElement {
    SelfClosingElement(HTMLStartTag),
    ElementWithChildren {
        start_tag: HTMLStartTag,
        end_tag: HTMLEndTag,
        children: Vec<HTMLChild>,
    },
}

fn html_char(input: &str) -> IResult<&str, char> {
    none_of("{<>}")(input)
}

fn html_text(input: &str) -> IResult<&str, HTMLChild> {
    let (input, result) = many1(html_char)(input)?;
    let result: String = result.into_iter().collect();

    Ok((input, HTMLChild::Text(result)))
}

fn html_element_wrapper(input: &str) -> IResult<&str, HTMLChild> {
    let (input, elem) = html_element(input)?;
    Ok((input, HTMLChild::Element(Box::new(elem))))
}

fn html_child(input: &str) -> IResult<&str, HTMLChild> {
    alt((html_text, html_element_wrapper))(input)
}

fn html_children(input: &str) -> IResult<&str, Vec<HTMLChild>> {
    many0(html_child)(input)
}

fn tag_name(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

fn attribute_char(input: &str) -> IResult<&str, char> {
    none_of("\"<>'/= \t\n\r")(input)
}

fn attribute_value_double_string(input: &str) -> IResult<&str, HTMLValue> {
    let (input, res) = delimited(char('"'), many0(none_of("\"")), char('"'))(input)?;
    let res: String = res.into_iter().collect();
    Ok((input, HTMLValue::String(res)))
}

fn attribute_value_single_string(input: &str) -> IResult<&str, HTMLValue> {
    let (input, res) = delimited(char('\''), many0(none_of("\'")), char('\''))(input)?;
    let res: String = res.into_iter().collect();
    Ok((input, HTMLValue::String(res)))
}

fn attribute_value(input: &str) -> IResult<&str, HTMLValue> {
    alt((attribute_value_double_string, attribute_value_single_string))(input)
}

fn attribute_name(input: &str) -> IResult<&str, String> {
    let (input, res) = many1(attribute_char)(input)?;
    let res: String = res.into_iter().collect();
    Ok((input, res))
}

// TODO: Add support for attributes without attribute values.
fn html_attribute(input: &str) -> IResult<&str, HTMLAttribute> {
    let (input, (_, name, _, _, _, value)) = tuple((
        multispace0,
        attribute_name,
        multispace0,
        char('='),
        multispace0,
        attribute_value,
    ))(input)?;
    Ok((input, HTMLAttribute::Attribute { name, value }))
}

fn html_attributes(input: &str) -> IResult<&str, Vec<HTMLAttribute>> {
    many0(html_attribute)(input)
}

fn opening_element(input: &str) -> IResult<&str, HTMLStartTag> {
    let (input, result) = tuple((
        multispace0,
        char('<'),
        tag_name,
        html_attributes,
        multispace0,
        char('>'),
    ))(input)?;
    let (_, _, name, attributes, _, _) = result;
    Ok((input, HTMLStartTag::Tag(name.to_string(), attributes)))
}

fn closing_element(input: &str) -> IResult<&str, HTMLEndTag> {
    let (input, result) = delimited(tag("</"), tag_name, char('>'))(input)?;
    Ok((input, HTMLEndTag::Tag(result.to_string())))
}

fn element_with_children(input: &str) -> IResult<&str, HTMLElement> {
    let (rem, (start_tag, children, end_tag)) =
        tuple((opening_element, html_children, closing_element))(input)?;

    let node = HTMLElement::ElementWithChildren {
        start_tag,
        children,
        end_tag,
    };

    Ok((rem, node))
}

fn self_closing_element(input: &str) -> IResult<&str, HTMLElement> {
    let (rem, results) =
        tuple((char('<'), tag_name, html_attributes, multispace0, tag("/>")))(input)?;
    let (_, identifier, attributes, _, _) = results;
    Ok((
        rem,
        HTMLElement::SelfClosingElement(HTMLStartTag::Tag(identifier.to_string(), attributes)),
    ))
}

pub fn html_element(input: &str) -> IResult<&str, HTMLElement> {
    alt((element_with_children, self_closing_element))(input)
}

#[cfg(test)]
mod tests;
