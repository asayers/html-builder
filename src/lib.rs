mod html;

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
    _phatom: std::marker::PhantomData<&'a ()>,
}

#[derive(Default)]
struct Ctx {
    wtr: String,
    stack: Vec<Cow<'static, str>>,
}

impl Root {
    pub fn new() -> Root {
        let ctx = Arc::new(Mutex::new(Ctx::default()));
        let node = Node {
            depth: 0,
            ctx: Arc::downgrade(&ctx),
            _phatom: std::marker::PhantomData,
        };
        Root { node, ctx }
    }

    pub fn build(self) -> String {
        let mutex = Arc::try_unwrap(self.ctx).ok().unwrap();
        let Ctx { mut wtr, mut stack } = mutex.into_inner().unwrap();
        while let Some(tag) = stack.pop() {
            wtr.push_str(&tag);
        }
        wtr
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

impl<'a> Node<'a> {
    pub fn tag<'b>(&'b mut self, tag: &str) -> Node<'b> {
        self.child(&format!("<{}>", tag), format!("</{}>", tag).into())
    }

    pub fn child<'b>(&'b mut self, open: &str, close: Cow<'static, str>) -> Node<'b> {
        let ctx = self.ctx.upgrade().unwrap();
        let mut ctx = ctx.lock().unwrap();
        let to_pop = ctx.stack.len() - self.depth;
        for _ in 0..to_pop {
            // TODO: More efficient impl?
            let tag = ctx.stack.pop().unwrap();
            ctx.wtr.push_str(&tag);
        }
        ctx.wtr.write_str(open).unwrap();
        ctx.stack.push(close);
        Node {
            depth: self.depth + 1,
            ctx: self.ctx.clone(),
            _phatom: std::marker::PhantomData,
        }
    }
}

impl<'a> Write for Node<'a> {
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.ctx
            .upgrade()
            .unwrap()
            .lock()
            .unwrap()
            .wtr
            .write_char(c)
    }
    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        self.ctx
            .upgrade()
            .unwrap()
            .lock()
            .unwrap()
            .wtr
            .write_fmt(args)
    }
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.ctx.upgrade().unwrap().lock().unwrap().wtr.write_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full() {
        let mut root = Root::new();
        let mut html = root.tag("html");
        let mut head = html.tag("head");
        let mut title = head.tag("title");
        write!(title, "Foobar").unwrap();
        let mut body = html.tag("body");
        write!(body, "Lorem ipsum").unwrap();
        assert_eq!(
            &root.build(),
            "<html><head><title>Foobar</title></head><body>Lorem ipsum</body></html>"
        );
    }

    #[test]
    fn elided() {
        let mut root = Root::new();
        let mut html = root.tag("html");
        write!(html.tag("head").tag("title"), "Foobar").unwrap();
        write!(html.tag("body"), "Lorem ipsum").unwrap();
        assert_eq!(
            &root.build(),
            "<html><head><title>Foobar</title></head><body>Lorem ipsum</body></html>"
        );
    }
}
