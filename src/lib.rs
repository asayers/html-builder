/*!

Short example:

```
# use pretty_assertions::assert_eq;
use html_builder::*;
use std::fmt::Write;

let mut buf = Buffer::new();                // Contents added to buffer by each statement:
let mut html = buf.html().attr("lang='en'");  // <html lang='en'>
writeln!(html.head().title(), "Title!")?;     // <head><title>Title!
writeln!(html.body().h1(), "Header!")?;       // </title></head><body><h1>Header!
let page = buf.finish();                       // </h1></body></html>
# assert_eq!(page, r#"<html lang='en'>
#  <head>
#   <title>
# Title!
#   </title>
#  </head>
#  <body>
#   <h1>
# Header!
#   </h1>
#  </body>
# </html>
# "#);
# Ok::<(), std::fmt::Error>(())
```

Longer example:

```
use html_builder::*;
use std::fmt::Write;

// Start by creating a Buffer.  This contains a buffer that we're going
// to be writing into.
let mut buf = Buffer::new();

// The buffer is writable
writeln!(buf, "<!-- My website -->")?;

// The Html5 trait provides various helper methods.  For instance, doctype()
// simply writes the <!DOCTYPE> header
buf.doctype();

// Most helper methods create child nodes.  You can set a node's attributes
// like so
let mut html = buf.html().attr("lang='en'");

let mut head = html.head();

// Just like Buffer, nodes are also writable.  Set their contents by
// writing into them.
writeln!(head.title(), "Website!")?;

// Meta is a "void element", meaning it doesn't need a closing tag.  This is
// handled correctly.
head.meta().attr("charset='utf-8'");

let mut body = html.body();
writeln!(body.h1(), "It's a website!")?;

// Generating HTML in a loop
let mut list = body.ul();
for i in 1..=3 {
    writeln!(
        list.li().a().attr(
            &format!("href='/page_{}.html'", i)
        ),
        "Page {}", i,
    )?
}

// You can write functions which add subtrees to a node
fn figure_with_caption(parent: &mut Node, src: &str, cap: &str) {
    let mut fig = parent.figure();
    fig.img()
        .attr(&format!("src='{}'", src))
        .attr(&format!("alt='{}'", cap));
    writeln!(fig.figcaption(), "{}", cap).unwrap();
}

figure_with_caption(&mut body, "img.jpg", "Awesome image");

// Text contents in an inner node
let mut footer = body.footer();
writeln!(footer, "Last modified")?;
writeln!(footer.time(), "2021-04-12")?;

// Finally, call finish() to extract the buffer.
buf.finish()
# ; Ok::<(), std::fmt::Error>(())
```
```html
<!-- My website -->
<!DOCTYPE>
<html lang='en'>
 <head>
  <title>Website!</title>
  <meta charset='utf-8'>
 </head>
 <body>
  <h1>It's a website!</h1>
  <ul>
   <li><a href='/page_1.html'>Page 1</a></li>
   <li><a href='/page_2.html'>Page 2</a></li>
   <li><a href='/page_3.html'>Page 3</a></li>
  </ul>
  <figure>
   <img src='img.jpg' alt='Awesome image'>
   <figcaption>Awesome image</figcaption>
  </figure>
  <footer>
   Last modified <time>2021-04-12</time>
  </footer>
 </body>
</html>
```

*/

mod html;
pub use html::*;

use std::borrow::Cow;
use std::fmt::Write;
use std::sync::{Arc, Mutex, Weak};

/// A buffer for writing HTML into.
pub struct Buffer {
    ctx: Arc<Mutex<Ctx>>,
    node: Node<'static>,
}

/// An HTML element.
pub struct Node<'a> {
    depth: usize,
    ctx: Weak<Mutex<Ctx>>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

/// A self-closing element.
///
/// Void elements can't have any contents (since there's no end tag, no
/// content can be put between the start tag and the end tag).
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

impl Buffer {
    /// Creates a new empty buffer.
    pub fn new() -> Buffer {
        Buffer::default()
    }

    /// Closes all open tags and returns the buffer's contents.
    pub fn finish(self) -> String {
        let mutex = Arc::try_unwrap(self.ctx).ok().unwrap();
        let mut ctx = mutex.into_inner().unwrap();
        ctx.close_deeper_than(0);
        ctx.wtr
    }
}

impl Default for Buffer {
    fn default() -> Buffer {
        let ctx = Arc::new(Mutex::new(Ctx::default()));
        let node = Node {
            depth: 0,
            ctx: Arc::downgrade(&ctx),
            _phantom: std::marker::PhantomData,
        };
        Buffer { node, ctx }
    }
}

impl std::ops::Deref for Buffer {
    type Target = Node<'static>;
    fn deref(&self) -> &Node<'static> {
        &self.node
    }
}

impl std::ops::DerefMut for Buffer {
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

    fn close_deeper_than(&mut self, depth: usize) {
        self.close_unclosed();
        let to_pop = self.stack.len() - depth;
        for _ in 0..to_pop {
            if let Some(tag) = self.stack.pop() {
                write!(self.wtr, "{:>w$}/{}>\n", "<", tag, w = self.stack.len() + 1).unwrap();
            }
        }
    }

    fn open(&mut self, tag: &str, depth: usize) {
        self.close_deeper_than(depth);
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
        ctx.close_deeper_than(self.depth);
        ctx.wtr.write_char(c)
    }
    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        let mutex = self.ctx.upgrade().unwrap();
        let mut ctx = mutex.lock().unwrap();
        ctx.close_deeper_than(self.depth);
        ctx.wtr.write_fmt(args)
    }
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let mutex = self.ctx.upgrade().unwrap();
        let mut ctx = mutex.lock().unwrap();
        ctx.close_deeper_than(self.depth);
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
    use pretty_assertions::assert_eq;

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
        let mut root = Buffer::new();
        let mut html = root.child("html".into());
        let mut head = html.child("head".into());
        let mut title = head.child("title".into());
        writeln!(title, "Foobar").unwrap();
        let mut body = html.child("body".into());
        writeln!(body, "Lorem ipsum").unwrap();
        assert_eq!(&root.finish(), EXPECTED);
    }

    #[test]
    fn elided() {
        let mut root = Buffer::new();
        let mut html = root.child("html".into());
        writeln!(html.child("head".into()).child("title".into()), "Foobar").unwrap();
        writeln!(html.child("body".into()), "Lorem ipsum").unwrap();
        assert_eq!(&root.finish(), EXPECTED);
    }

    #[test]
    fn pre_post_inner() {
        let mut buf = Buffer::new();
        let mut a = buf.child("a".into());
        writeln!(a, "a pre").unwrap();
        let mut b = a.child("b".into());
        writeln!(b, "b pre").unwrap();
        let mut c = b.child("c".into());
        writeln!(c, "c pre").unwrap();
        writeln!(c, "c post").unwrap();
        writeln!(b, "b post").unwrap();
        writeln!(a, "a post").unwrap();
        assert_eq!(
            buf.finish(),
            "\
<a>
a pre
 <b>
b pre
  <c>
c pre
c post
  </c>
b post
 </b>
a post
</a>
"
        );
    }
}
