use criterion::{criterion_group, criterion_main, Criterion};
use html_builder::*;
use std::fmt::Write;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("sample html", |b| b.iter(|| sample_html().unwrap()));
}

fn sample_html() -> Result<String, Box<dyn std::error::Error>> {
    let mut buf = Buffer::new();
    buf.doctype();
    let mut html = buf.html().attr("lang='en'");
    let mut head = html.head();
    writeln!(head.title(), "Website!")?;
    head.meta().attr("charset='utf-8'");
    let mut body = html.body();
    writeln!(body.h1(), "It's a website!")?;
    let mut list = body.ul();
    for i in 0..2 {
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
    writeln!(body.comment(), "Thanks for reading")?;
    let page = buf.finish();
    Ok(page)
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
