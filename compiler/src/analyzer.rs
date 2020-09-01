use crate::parser::*;

pub fn analyze_tree(tree: &HTMLElement) -> Result<(), String> {
  match tree {
    HTMLElement::ElementWithChildren { start_tag, end_tag, children } => {
      let HTMLStartTag::Tag(start_name, _) = start_tag;
      let HTMLEndTag::Tag(end_name) = end_tag;

      if !start_name.to_lowercase().eq(&end_name.to_lowercase()) {
        return Err(format!("Start and end tag are not equal. Start tag: {}, end tag: {}", start_name, end_name));
      }

      analyze_children(children)
    },
    HTMLElement::SelfClosingElement(_) => Ok(())
  }
}

pub fn analyze_children(children: &Vec<HTMLChild>) -> Result<(), String> {
  for child in children.iter() {
    match child {
      HTMLChild::Element(elem) => {
        let elem = elem.as_ref();
        analyze_tree(elem)?
      }
      HTMLChild::Text(_) => {}
    }
  }

  Ok(())
}