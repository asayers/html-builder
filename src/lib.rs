mod html;
pub use html::*;

use std::borrow::Cow;
use std::fmt::Write;
use std::sync::{Arc, Mutex, Weak};

#[derive(Clone)]
pub struct Root {
    ctx: Arc<Mutex<Ctx>>,
    node: Node<'static>,
}

#[derive(Clone)]
pub struct Node<'a> {
    depth: usize,
    ctx: Weak<Mutex<Ctx>>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

#[derive(Default)]
struct Ctx {
    wtr: String,
    stack: Vec<Cow<'static, str>>,
    tag_open: bool,
}

impl Root {
    pub fn new() -> Root {
        let ctx = Arc::new(Mutex::new(Ctx::default()));
        let node = Node {
            depth: 0,
            ctx: Arc::downgrade(&ctx),
            _phantom: std::marker::PhantomData,
        };
        Root { node, ctx }
    }

    pub fn build(self) -> String {
        let mutex = Arc::try_unwrap(self.ctx).ok().unwrap();
        let mut ctx = mutex.into_inner().unwrap();
        ctx.close_unclosed();
        while let Some(tag) = ctx.stack.pop() {
            write!(ctx.wtr, "</{}>", tag).unwrap();
        }
        ctx.wtr
    }
}

impl std::ops::Deref for Root {
    type Target = Node<'static>;
    fn deref(&self) -> &Node<'static> {
        &self.node
    }
}

impl std::ops::DerefMut for Root {
    fn deref_mut(&mut self) -> &mut Node<'static> {
        &mut self.node
    }
}

impl Ctx {
    fn close_unclosed(&mut self) {
        if self.tag_open {
            self.tag_open = false;
            self.wtr.write_str(">").unwrap();
        }
    }
}

impl<'a> Node<'a> {
    pub fn child<'b>(&'b mut self, tag: Cow<'static, str>) -> Node<'b> {
        let ctx = self.ctx.upgrade().unwrap();
        let mut ctx = ctx.lock().unwrap();
        ctx.close_unclosed();
        let to_pop = ctx.stack.len() - self.depth;
        for _ in 0..to_pop {
            // TODO: More efficient impl?
            let tag = ctx.stack.pop().unwrap();
            write!(ctx.wtr, "</{}>", tag).unwrap();
        }
        write!(ctx.wtr, "<{}", tag).unwrap();
        ctx.stack.push(tag);
        ctx.tag_open = true;
        Node {
            depth: self.depth + 1,
            ctx: self.ctx.clone(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn attr(&mut self, attr: &str) {
        let ctx = self.ctx.upgrade().unwrap();
        let mut ctx = ctx.lock().unwrap();
        if ctx.tag_open {
            write!(ctx.wtr, " {}", attr).unwrap();
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full() {
        let mut root = Root::new();
        let mut html = root.child("html".into());
        let mut head = html.child("head".into());
        let mut title = head.child("title".into());
        write!(title, "Foobar").unwrap();
        let mut body = html.child("body".into());
        write!(body, "Lorem ipsum").unwrap();
        assert_eq!(
            &root.build(),
            "<html><head><title>Foobar</title></head><body>Lorem ipsum</body></html>"
        );
    }

    #[test]
    fn elided() {
        let mut root = Root::new();
        let mut html = root.child("html".into());
        write!(html.child("head".into()).child("title".into()), "Foobar").unwrap();
        write!(html.child("body".into()), "Lorem ipsum").unwrap();
        assert_eq!(
            &root.build(),
            "<html><head><title>Foobar</title></head><body>Lorem ipsum</body></html>"
        );
    }
}
