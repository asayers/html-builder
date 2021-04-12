use std::fmt::Write;
use std::sync::{Arc, Mutex, Weak};

#[derive(Clone)]
pub struct Root {
    ctx: Arc<Mutex<Ctx>>,
    node: Node,
}

#[derive(Clone)]
pub struct Node {
    depth: usize,
    ctx: Weak<Mutex<Ctx>>,
}

#[derive(Default)]
struct Ctx {
    wtr: String,
    stack: Vec<String>,
}

impl Root {
    pub fn new() -> Root {
        let ctx = Arc::new(Mutex::new(Ctx::default()));
        let node = Node {
            depth: 0,
            ctx: Arc::downgrade(&ctx),
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
    type Target = Node;
    fn deref(&self) -> &Node {
        &self.node
    }
}

impl Node {
    pub fn tag(&self, tag: &str) -> Node {
        self.child(&format!("<{}>", tag), format!("</{}>", tag))
    }

    pub fn child(&self, open: &str, close: String) -> Node {
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
        }
    }
}

impl Write for Node {
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
    #[ignore]
    fn bad() {
        let root = Root::new();
        let mut a = root.tag("a");
        root.tag("b");
        write!(a, "Inner of a").unwrap();
        assert_eq!(&root.build(), "<a>Inner of a</a><b></b>");
    }

    #[test]
    #[should_panic]
    fn use_after_build() {
        let root = Root::new();
        let a = root.tag("a");
        root.build();
        a.tag("b");
    }

    #[test]
    #[ignore]
    fn even_worse() {
        let root = Root::new();
        let a = root.tag("a");
        root.tag("b");
        a.tag("c");
        assert_eq!(&root.build(), "<a><c></c></a><b></b>");
    }

    #[test]
    fn full() {
        let root = Root::new();
        let html = root.tag("html");
        let head = html.tag("head");
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
        let root = Root::new();
        let html = root.tag("html");
        write!(html.tag("head").tag("title"), "Foobar").unwrap();
        write!(html.tag("body"), "Lorem ipsum").unwrap();
        assert_eq!(
            &root.build(),
            "<html><head><title>Foobar</title></head><body>Lorem ipsum</body></html>"
        );
    }
}
