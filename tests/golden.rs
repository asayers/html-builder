use html_builder::*;
use std::fmt::Write;

#[test]
fn comment() -> std::fmt::Result {
    let mut buf = Buffer::new();
    let mut node = buf.child("node 1".into());
    let mut comment = node.comment();
    write!(comment, "comment 1A")?;
    let mut comment = node.comment();
    write!(comment, "comment 1B")?;
    let mut node = node.child("node 2".into());
    let mut comment = node.comment();
    write!(comment, "comment 2")?;
    let mut comment = buf.comment();
    write!(comment, "comment 3")?;
    insta::assert_snapshot!(buf.finish());
    Ok(())
}

#[test]
fn from_readme() -> std::fmt::Result {
    let mut buf = Buffer::new();
    buf.doctype();
    writeln!(buf, "<!-- My website -->")?;
    let mut html = buf.html().attr("lang='en'");
    let mut head = html.head();
    writeln!(head.title(), "Website!")?;
    head.meta().attr("charset='utf-8'");
    let mut body = html.body();
    writeln!(body.h1(), "It's a website!")?;
    let mut list = body.ul();
    for i in 1..=3 {
        writeln!(
            list.li().a().attr(&format!("href='/page_{}.html'", i)),
            "Page {}",
            i,
        )?
    }
    fn figure_with_caption(parent: &mut Node, src: &str, cap: &str) {
        let mut fig = parent.figure();
        fig.img()
            .attr(&format!("src='{}'", src))
            .attr(&format!("alt='{}'", cap));
        writeln!(fig.figcaption(), "{}", cap).unwrap();
    }
    figure_with_caption(&mut body, "img.jpg", "Awesome image");
    let mut footer = body.footer();
    writeln!(footer, "Last modified")?;
    writeln!(footer.time(), "2021-04-12")?;
    insta::assert_snapshot!(buf.finish());
    Ok(())
}

#[test]
fn full() {
    let mut root = Buffer::new();
    let mut html = root.child("html".into());
    let mut head = html.child("head".into());
    let mut title = head.child("title".into());
    writeln!(title, "Foobar").unwrap();
    let mut body = html.child("body".into());
    writeln!(body, "Lorem ipsum").unwrap();
    insta::assert_snapshot!(root.finish());
}

#[test]
fn elided() {
    let mut root = Buffer::new();
    let mut html = root.child("html".into());
    writeln!(html.child("head".into()).child("title".into()), "Foobar").unwrap();
    writeln!(html.child("body".into()), "Lorem ipsum").unwrap();
    insta::assert_snapshot!(root.finish());
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
    insta::assert_snapshot!(buf.finish());
}
