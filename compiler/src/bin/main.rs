use compiler::parser::html_element;
use compiler::analyzer::analyze_tree;

fn main() {
    let html = "<div>
        Testing this out
        <h1>
            This works!
            <div class='testing' anotherOne   ='this works!'>Sub element</div>
            <div/>
        </h1>
        <div>This works as well!</other>
    </div>";
    let (_, results) = html_element(html).unwrap();
    let other_results = analyze_tree(&results);

    match other_results {
      Ok(()) => {
        println!("{:#?}", results);
      }
      Err(string) => {
        println!("{}", string);
      }
    }
}