# handlebars

This is the first version of a Rust-based compiler and VM for handlebars. This is highly experimental and it's not recommended to use this in production just yet.
The compiler uses [Nom](https://github.com/Geal/nom), a strongly-typed parser combinator library for Rust, to parse 'HTML'. The grammar for the HTML is based off of 
[the grammar for JSX](https://facebook.github.io/jsx/), an extension of ES6's grammar that allows you to write XML-like syntax in Javascript code.

Currently the lexer of the compiler supports the following:

- Self-closing HTML tags (eg. `<div/>`)
- HTML tags with children (eg. `<div></div>`)
- JSX text as a child (any string that does not contain the characters `<>{}`)
- HTML elements as a child (eg. `<div><h1>Hi!</h1></div>`)

The lexer uses strongly typed enums to build the AST. Because this uses a parser combinator library, the lexer vaidates quite a lot of the valid HTML syntax already. 
However, certains things require an additional verification pass after initial parsing (or will require additional error handling to be written into the combinator
functions). Errors like mismatching HTML tags are not checked currently.