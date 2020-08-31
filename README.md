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
    println!("{:?}", item);
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
println!("{:?}", item);
```

This uses the `println!` macro (basically a function, but slightly more complex) to output the value of `item`. A call to `println!` will always begin with a string like that; inside the string, you can see we're using a special syntax: `{:?}`. This little bracket thingy basically takes whatever argument is passed into `println!` and outputs all of it (so long as it implements the `Debug` trait, but that's not important). If you've used `println` in any other language, the behavior is almost identical, but the syntax is just a little bit different.

So that's what our code does! It takes the stuff stored in the `html` variable, parses it, and outputs it to the console. Play around with it and see what happens. Here's some experiments you can try:

- What happens when you enter invalid HTML?
- What happens when you enter an empty string?
- What happens if you remove a variable?
- What happens if you remove one of the semicolons?
- What happens if you slightly misspell the `html` variable?

Those last three are going to give you *compiler errors*, because the Rust compiler is very particular about what it will let you do. I'd definitely recommend trying to trigger one (only those two, though, other errors can get gnarly if you don't know what you're doing) to get an idea of what compiler errors in Rust look like. Notice how friendly they are?

That's what's great about Rust. The programs you right with it are more stable, easier to reason about, and are *much* faster than most other programming languages.
Rust is a systems-level programming langauge with a lot of higher-level features and packages, meaning you can use it basically anywhere in your stack. It integrates
with Javascript and Ruby, can run in the browser using WASM, can run on almost any computer, and can do a lot of cool stuff that you'll miss when you're
using other langauges.

Also one of the more talked about features with Rust is that it doesn't have a garbage compiler, making it as fast as C. However, you never have to manually
allocate memory with Rust because of its unique *ownership model*. If your code compiles, it's memory efficient and de-allocates variables when it needs to.
If you want to learn more about Rust and why it's the most-loved programming language on StackOverflow for four years in row, [read this article](https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/#:~:text=One%20of%20the%20biggest%20benefits,and%20can%20be%20cleaned%20up.).