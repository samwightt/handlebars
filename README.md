# handlebars

This is the first version of a Rust-based compiler and VM for handlebars. This is highly experimental and it's not recommended to use this in production just yet.
The compiler uses [Nom](https://github.com/Geal/nom), a strongly-typed parser combinator library for Rust, to parse 'HTML'. The grammar for the HTML is based off of 
[the grammar for JSX](https://facebook.github.io/jsx/), an extension of ES6's grammar that allows you to write XML-like syntax in Javascript code.

Currently the lexer of the compiler supports the following:

- Self-closing HTML tags (eg. `<div/>`)
- HTML tags with children (eg. `<div></div>`)
- JSX text as a child (any string that does not contain the characters `<>{}`)
- HTML elements as a child (eg. `<div><h1>Hi!</h1></div>`)

The lexer uses strongly typed enums to build the AST. Because this uses a parser combinator library, the lexer vaidates quite a lot of the HTML syntax already. 
However, certains things require an additional verification pass after initial parsing (or will require additional error handling to be written into the combinator
functions). Errors like mismatching HTML tags are not checked currently.

## Installation

To start, make sure you have the latest version of Rust installed. [If you don't have Rust installed, you can download it here](https://www.rust-lang.org/tools/install).

After you install Rust, clone the repository and `cd` into the directory:

```
git clone https://github.com/samwightt/handlebars
cd handlebars
```

To run the example code, cd into the `compiler` directory:

```
cd compiler
```

To run the compiler, run the following:

```
cargo run
```

This will automatically install all of the necessary packages (called *crates* in Rust), build the binary, and run it. You should see something like the following:

```
Finished dev [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/compiler`
Ok(("", ElementWithChildren { start_tag: TagName("div"), end_tag: TagName("div"), children: [Text("\n        Testing this out\n        "), Element(ElementWithChildren { start_tag: TagName("h1"), end_tag: TagName("h1"), children: [Text("\n            This works!\n            "), Element(ElementWithChildren { start_tag: TagName("div"), end_tag: TagName("div"), children: [Text("Sub element")] }), Text("\n            "), Element(SelfClosingElement(TagName("div"))), Text("\n        ")] }), Text("\n        "), Element(ElementWithChildren { start_tag: TagName("div"), end_tag: TagName("div"), children: [Text("This works as well!")] }), Text("\n    ")] }))
```

## Explanation

This is the AST for the HTML in the `main.rs` file. You can ignore the surrounding `Ok` enum, as well as the first string. The real meat of everything is in the `ElementWithChildren` enum. It's a little rough to look at right now, but you can see that the entire content of the HTML has been converted to the enums defined in `main.rs`.

To try the compiler with a different piece of HTML, open `main.rs` and scroll down to the bottom of the file. You should see something like the following:

```
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
    println!("{:#?}", item);
}
```

Let's walk through what this does. `fn main() { // Code }` is a special function in Rust that gets called when you execute a binary. Think of it like the `main()` function in C or the `public static void main(String[] args)` in Java. It's the exact same thing.

Each statement in a Rust program ends with a semicolon (except for some, but don't worry about that). Statements can be multiline. The first statement in the function is this:

```
let html = "<div>
        Testing this out
        <h1>
            This works!
            <div>Sub element</div>
            <div/>
        </h1>
        <div>This works as well!</div>
    </div>";
```

Rust uses the `let` keyword to define a variable. In Rust, everything is very strictly typed, however, type definitions are optional in a lot of places; variable declarations are almost always one of those. Here, we're defining our HTML string that the compiler will parse. Strings are multiline in Rust by default.

Change the HTML to something like the following:

```
let html = "<div/>";
```

It can be as simple as this or much more complex. Let's look at what the next line of code does:

```
let item = element(html);
```

This calls the HTML parser, passing in the HTML string we just defined and storing the results in the `item` variable. Again note how type declarations are optional here. However, if we need to get the type, we can just scroll up:

```
fn element(input: &str) -> IResult<&str, HTMLElement> {
    // Other code
}
```

Element is a function that accepts in a string reference (`&str`) and returns a pretty gnarly looking type, `IResult<&str, HTMLElement>`. Don't worry about what this means; all you need to know is that when the function parses the HTML successfully, you'll see an `Ok` outputted in the terminal. When there's an error, you'll see `Err`.

Let's look at the last line of code:

```
println!("{:#?}", item);
```

This uses the `println!` macro (basically a function, but slightly more complex) to output the value of `item`. A call to `println!` will always begin with a string like that; inside the string, you can see we're using a special syntax: `{:#?}`. This little bracket thingy basically takes whatever argument is passed into `println!` and outputs all of it (so long as it implements the `Debug` trait, but that's not important). If you've used `println` in any other language, the behavior is almost identical, but the syntax is just a little bit different.

So that's what our code does! It takes the stuff stored in the `html` variable, parses it, and outputs it to the console. Play around with it and see what happens. Here's some experiments you can try:

- What happens when you enter invalid HTML?
- What happens when you enter an empty string?
- What happens if you remove a variable?
- What happens if you remove one of the semicolons?
- What happens if you slightly misspell the `html` variable?

Those last three are going to give you *compiler errors*, because the Rust compiler is very particular about what it will let you do. I'd definitely recommend trying to trigger one (only those three, though, other errors can get gnarly if you don't know what you're doing) to get an idea of what compiler errors in Rust look like. Notice how friendly they are?

That's what's great about Rust. The programs you write with it are more stable, easier to reason about, and are *much* faster than most other programming languages.
Rust is a systems-level programming langauge with a lot of higher-level features and packages, meaning you can use it basically anywhere in your stack. It integrates
with Javascript and Ruby, can run in the browser using WASM, can run on almost any computer, and can do a lot of cool stuff that you'll miss when you're
using other langauges.

Also one of the more talked about features with Rust is that it doesn't have a garbage collector, making it as fast as C. However, you never have to manually
allocate memory with Rust because of its unique *ownership model*. If your code compiles, it's memory efficient and de-allocates variables when it needs to.
If you want to learn more about Rust and why it's the most-loved programming language on StackOverflow for four years in row, [read this article](https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/#:~:text=One%20of%20the%20biggest%20benefits,and%20can%20be%20cleaned%20up.).

## What are parser combinators?

Glad you asked! Parser combinators are basically reusable functions that accept in raw data that a computer understands and converts it to something that a computer 
does understand. What makes them cool is that they're *composable*. You can define really complex parsing logic (often called *grammar* with programming languages)
by starting with the smallest parts of it and piecing the results together like Legos. 

Let's look at an example. Here's the code for a combinator called `html_char` in the parser here:

```
fn html_char(input: &str) -> IResult<&str, char> {
    none_of("{<>}")(input)
}
```

Remember how we defined `fn main()` earlier? Well we can define other functions in Rust using the same syntax. Here we define a function called `html_char` that
accepts in a string reference (that `&str`, remember) and returns another one of those `IResult` things. Note that Rust has a weird return syntax: if you don't put
a semicolon (`;`) on the last line of a function, Rust will assume that that's what you're meaning to return (called *implicit returns*, much like Ruby). You can
still use the regular `return` keyword that you're used to though:

```
return none_of("{<>}")(input);
```

What is this code doing though? Well, we're using a parser combinator provided by the `nom` library. This one (called `none_of`) accepts in a string of characters
to not match with. It returns a function that can be used to validate whether a character is not one of these characters. Here, we're validating `input`, our input arg,
and returning the result. 

Note that input is a string, though. What happens to the rest of the characters if the first one gets consumed? Well, they're passed back in that `IResult`. Let's look again that this return type:

```
IResult<&str, char>
```

The first part of the IResult is **what's left over from whatever parsing we're doing**. The second part is **what is parsed**. So here, we're parsing a `char` (a 
character in Rust) and returning it, then also returning *all of the leftover string*. That's how we can *combine* the parser combinators; because we always
have the leftovers of them, we can just pass those leftovers into other parser combinators.

Let's look at the `html_text` parser combinator in the `main.rs` file next:

```
fn html_text(input: &str) -> IResult<&str, HTMLChild> {
    let (input, result) = many1(html_char)(input)?;
    let result: String = result.into_iter().collect();

    Ok((input, HTMLChild::Text(result)))
}
```

It's not too important to understand how this works. All you need to look at is this first line:

```
let (input, result) = many1(html_char)(input)?;
```

We're using our previous parser combinator that we just defined, `html_char`, and passing it into another combinator defined by `nom`, `many`, which matches one or more occurences of a parser combinator. Basically, it'll take our `html_char`, run it over and over and over again on those leftovers from the `input`, wait until it fails, and then return the results in that `results` variables. The `input` variable there is still our leftovers. Super cool, right?

This means that we can define our grammar for our Handlebars variant using these small little functions, and then we can sort of call them recursively to get an AST
([Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree)). This simple code can parse any sort of complex HTML we throw at it, in a completely type-safe way. Pretty cool, right?

If you want to learn more about grammars, lexers, compilers, interpreters, and the like, I'd highly recommend checking out the book [Crafting Interpreters](https://craftinginterpreters.com/),
a book that teaches you how to write your own interpreter. The part about grammars is where it gets really important, but scanning is also important to understand. I'd also recommend
following my dev friend [Linus on Twitter](https://twitter.com/thesephist); he makes his own programming language, Ink, and tweets a lot about it. He's the person that originally got me
into making programming langauges.