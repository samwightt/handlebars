use nom::{IResult, character::complete::{char, alpha1}, sequence::delimited, bytes::complete::tag};

fn start_tag(input: &str) -> IResult<&str, &str> {
    delimited(char('<'), alpha1, char('>'))(input)
}

fn end_tag(input: &str) -> IResult<&str, &str> {
    delimited(tag("</"), alpha1, char('>'))(input)
}

fn html_tag(input: &str) -> IResult<&str, &str> {
    delimited(start_tag, alpha1, end_tag)(input)
}

fn main() {
    let result = html_tag("<href>asdf</href>");
    match result {
        Ok((first, second)) => println!("{}", second),
        Err(_) => println!("Didn't work!")
    }
    println!("This works!");
}