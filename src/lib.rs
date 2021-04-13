/*! Auto-closing tags for convenient HTML generation

This crate is a helper for code which generates HTML.  It makes
the process somewhat more tolerable than writing the tags by hand by
automatically closing tags for you.  Not only is this more convenient,
it guarantees a well-formed tree.  It doesn't do any fancy stuff like
[typed-html](https://docs.rs/typed-html), so it's still up to you to write
valid HTML.  IMO it strikes a good balance of safely to simplicity/flexibility.

```
# use pretty_assertions::assert_eq;
use html_builder::*;
use std::fmt::Write;

// Start by creating a Document.  This contains a String that we're going
// to be writing into.
let mut doc = Document::new();

// Some methods simply write into the internal buffer
doc.doctype();

// Most helper methods create child nodes.  You can set a node's attributes
// like so
let mut html = doc.html().attr("lang='en'");

let mut head = html.head();

// Meta is a "void element", meaning it doesn't need a closing tag.  This is
// handled correctly.
head.meta().attr("charset='utf-8'");

// Nodes implement Write.  Set their contents by writing into them.
writeln!(head.title(), "Website!")?;

let mut body = html.body();
writeln!(body.h1(), "It's a website!")?;

// Generating HTML in a loop
let mut list = body.ul();
for i in 0..2 {
    writeln!(
        list.li().a().attr(
            &format!("href='/page_{}.html'", i)
        ),
        "Page {}", i,
    )?
}

// Text contents in an inner node
let mut footer = body.footer();
writeln!(footer, "Last modified")?;
writeln!(footer.time(), "2021-04-12")?;

// Finally, call build() to extract the buffer.
let page = doc.build();

assert_eq!(
    page,
    r#"<!DOCTYPE>
<html lang='en'>
 <head>
  <meta charset='utf-8'>
  <title>
Website!
  </title>
 </head>
 <body>
  <h1>
It's a website!
  </h1>
  <ul>
   <li>
    <a href='/page_0.html'>
Page 0
    </a>
   </li>
   <li>
    <a href='/page_1.html'>
Page 1
    </a>
   </li>
  </ul>
  <footer>
Last modified
   <time>
2021-04-12
   </time>
  </footer>
 </body>
</html>
"#);
# Ok::<(), std::fmt::Error>(())
```

*/

mod html;
pub use html::*;

use std::borrow::Cow;
use std::fmt::Write;
use std::sync::{Arc, Mutex, Weak};

/// An HTML document.
#[derive(Clone)]
pub struct Document {
    ctx: Arc<Mutex<Ctx>>,
    node: Node<'static>,
}

/// An HTML element.
#[derive(Clone)]
pub struct Node<'a> {
    depth: usize,
    ctx: Weak<Mutex<Ctx>>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

/// A self-closing element.
///
/// Void elements can't have any contents (since there's no end tag, no
/// content can be put between the start tag and the end tag).
#[derive(Clone)]
pub struct Void<'a> {
    ctx: Weak<Mutex<Ctx>>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Default)]
struct Ctx {
    wtr: String,
    stack: Vec<Cow<'static, str>>,
    tag_open: bool,
}

impl Document {
    pub fn new() -> Document {
        let ctx = Arc::new(Mutex::new(Ctx::default()));
        let node = Node {
            depth: 0,
            ctx: Arc::downgrade(&ctx),
            _phantom: std::marker::PhantomData,
        };
        Document { node, ctx }
    }

    pub fn build(self) -> String {
        let mutex = Arc::try_unwrap(self.ctx).ok().unwrap();
        let mut ctx = mutex.into_inner().unwrap();
        ctx.close_unclosed();
        while !ctx.stack.is_empty() {
            ctx.pop();
        }
        ctx.wtr
    }
}

impl std::ops::Deref for Document {
    type Target = Node<'static>;
    fn deref(&self) -> &Node<'static> {
        &self.node
    }
}

impl std::ops::DerefMut for Document {
    fn deref_mut(&mut self) -> &mut Node<'static> {
        &mut self.node
    }
}

impl Ctx {
    fn close_unclosed(&mut self) {
        if self.tag_open {
            self.tag_open = false;
            self.wtr.write_str(">\n").unwrap();
        }
    }

    fn pop(&mut self) {
        let depth = self.stack.len();
        if let Some(tag) = self.stack.pop() {
            write!(self.wtr, "{:>w$}/{}>\n", "<", tag, w = depth).unwrap();
        }
    }

    fn open(&mut self, tag: &str, depth: usize) {
        self.close_unclosed();
        let to_pop = self.stack.len() - depth;
        for _ in 0..to_pop {
            self.pop();
        }
        write!(self.wtr, "{:>w$}{}", "<", tag, w = depth + 1).unwrap();
        self.tag_open = true;
    }
}

impl<'a> Node<'a> {
    pub fn child<'b>(&'b mut self, tag: Cow<'static, str>) -> Node<'b> {
        let ctx = self.ctx.upgrade().unwrap();
        let mut ctx = ctx.lock().unwrap();
        ctx.open(&tag, self.depth);
        ctx.stack.push(tag);
        Node {
            depth: self.depth + 1,
            ctx: self.ctx.clone(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn void_child<'b>(&'b mut self, tag: Cow<'static, str>) -> Void<'b> {
        let ctx = self.ctx.upgrade().unwrap();
        let mut ctx = ctx.lock().unwrap();
        ctx.open(&tag, self.depth);
        Void {
            ctx: self.ctx.clone(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn attr(self, attr: &str) -> Node<'a> {
        let ctx = self.ctx.upgrade().unwrap();
        let mut ctx = ctx.lock().unwrap();
        if ctx.tag_open {
            write!(ctx.wtr, " {}", attr).unwrap();
        }
        self
    }
}

impl<'a> Write for Node<'a> {
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        let mutex = self.ctx.upgrade().unwrap();
        let mut ctx = mutex.lock().unwrap();
        ctx.close_unclosed();
        ctx.wtr.write_char(c)
    }
    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        let mutex = self.ctx.upgrade().unwrap();
        let mut ctx = mutex.lock().unwrap();
        ctx.close_unclosed();
        ctx.wtr.write_fmt(args)
    }
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let mutex = self.ctx.upgrade().unwrap();
        let mut ctx = mutex.lock().unwrap();
        ctx.close_unclosed();
        ctx.wtr.write_str(s)
    }
}

impl<'a> Void<'a> {
    pub fn attr(self, attr: &str) -> Void<'a> {
        let ctx = self.ctx.upgrade().unwrap();
        let mut ctx = ctx.lock().unwrap();
        if ctx.tag_open {
            write!(ctx.wtr, " {}", attr).unwrap();
        }
        self
    }
}

impl<'a> Write for Void<'a> {
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        let mutex = self.ctx.upgrade().unwrap();
        let mut ctx = mutex.lock().unwrap();
        ctx.close_unclosed();
        ctx.wtr.write_char(c)
    }
    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        let mutex = self.ctx.upgrade().unwrap();
        let mut ctx = mutex.lock().unwrap();
        ctx.close_unclosed();
        ctx.wtr.write_fmt(args)
    }
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let mutex = self.ctx.upgrade().unwrap();
        let mut ctx = mutex.lock().unwrap();
        ctx.close_unclosed();
        ctx.wtr.write_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED: &str = "\
<html>
 <head>
  <title>
Foobar
  </title>
 </head>
 <body>
Lorem ipsum
 </body>
</html>
";

    #[test]
    fn full() {
        let mut root = Document::new();
        let mut html = root.child("html".into());
        let mut head = html.child("head".into());
        let mut title = head.child("title".into());
        writeln!(title, "Foobar").unwrap();
        let mut body = html.child("body".into());
        writeln!(body, "Lorem ipsum").unwrap();
        assert_eq!(&root.build(), EXPECTED);
    }

    #[test]
    fn elided() {
        let mut root = Document::new();
        let mut html = root.child("html".into());
        writeln!(html.child("head".into()).child("title".into()), "Foobar").unwrap();
        writeln!(html.child("body".into()), "Lorem ipsum").unwrap();
        assert_eq!(&root.build(), EXPECTED);
    }
}
