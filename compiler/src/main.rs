use nom::{IResult, combinator::not, character::complete::{char, alphanumeric1, none_of, multispace0}, sequence::{tuple, delimited}, bytes::complete::tag, branch::{alt}, multi::{many1, many0}};

#[derive(Debug)]
enum HTMLChild {
    Text(String),
    Element(Box<HTMLElement>)
}

#[derive(Debug)]
enum HTMLIdentifier {
    TagName(String),
    AttributeName(String)
}

#[derive(Debug)]
enum HTMLValue {
    String(String)
}

#[derive(Debug)]
enum HTMLAttribute {
    Attribute {
        name: HTMLIdentifier,
        value: HTMLValue
    }
}

#[derive(Debug)]
enum HTMLElement {
    SelfClosingElement(HTMLIdentifier),
    ElementWithChildren {
        start_tag: HTMLIdentifier,
        end_tag: HTMLIdentifier,
        children: Vec<HTMLChild>
    }
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
    let (input, elem) = element(input)?;
    Ok((input, HTMLChild::Element(Box::new(elem))))
}

fn html_child(input: &str) -> IResult<&str, HTMLChild> {
    alt((html_text, html_element_wrapper))(input)
}

fn html_children(input: &str) -> IResult<&str, Vec<HTMLChild>> {
    many0(html_child)(input)
}

fn tag_name(input: &str) -> IResult<&str, HTMLIdentifier> {
    let (input, result) = alphanumeric1(input)?;
    Ok((input, HTMLIdentifier::TagName(result.to_string())))
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

fn attribute_name(input: &str) -> IResult<&str, HTMLIdentifier> {
    let (input, res) = many0(attribute_char)(input)?;
    let res: String = res.into_iter().collect();
    return Ok((input, HTMLIdentifier::AttributeName(res)))
}

fn html_attribute(input: &str) -> IResult<&str, HTMLAttribute> {
    let (input, (_, name, _, _, _, value)) = tuple((multispace0, attribute_name, multispace0, char('='), multispace0, attribute_value))(input)?;
    Ok((input, HTMLAttribute::Attribute { name, value }))
}

fn opening_element(input: &str) -> IResult<&str, HTMLIdentifier> {
    let (input, result) = tuple((multispace0, char('<'), tag_name, char('>')))(input)?;
    let (_, _, result, _) = result;
    Ok((input, result))
}

fn closing_element(input: &str) -> IResult<&str, HTMLIdentifier> {
    delimited(tag("</"), tag_name, char('>'))(input)
}

fn element_with_children(input: &str) -> IResult<&str, HTMLElement> {
    let (input, (start_tag, children, end_tag)) = tuple((opening_element, html_children, closing_element))(input)?;

    let node = HTMLElement::ElementWithChildren {
        start_tag,
        children,
        end_tag
    };

    Ok((input, node))
}

fn self_closing_element(input: &str) -> IResult<&str, HTMLElement> {
    let (input, identifier) = delimited(char('<'), tag_name, tag("/>"))(input)?;
    Ok((input, HTMLElement::SelfClosingElement(identifier)))
}

fn element(input: &str) -> IResult<&str, HTMLElement> {
    alt((element_with_children, self_closing_element))(input)
}

fn main() {
    let html = "<div>
        Testing this out
        <h1>
            This works!
            <div>Sub element</div>
            <div/>
        </h1>
        <div>This works as well!</div>
    </div>";
    let item = element(html);
    println!("{:?}", item);

    println!("This works!");
}