# html-builder

This crate helps you generate HTML.

```rust
let mut doc = Document::new();                // Contents added to buffer by each statement:
let mut html = doc.html().attr("lang='en'");  // <html lang='en'>
writeln!(html.head().title(), "Title!")?;     // <head><title>Title!
writeln!(html.body().h1(), "Header!")?;       // </title></head><body><h1>Header!
doc.build();                                  // </h1></body></html>
```

* It automatically closes your tags for you.  This makes code nicer to write,
  nicer to read, and harder to screw up (it guarantees you get a well-formed
  tree).
* It also provides helper methods for all the valid HTML5 tags.  You don't
  have to use these, but if you do it makes sure you don't typo a tag name.

IMO it strikes a good balance of safety to simplicity/flexibility.

It's by no means built for performance, but nevertheless the performance
is OK I think.  The benchmark generates a small (20-line) HTML document.
On my laptop it runs in 4us.  Run `cargo bench` to check for yourself.

There are high quality templating libraries out there, such as [askama],
but personally I don't like templates.  [typed-html] is closer to what I
want, and it aims for a high level of type safety (eg. it forbids div-level
elements inside span-level elements).  This is great in theory, but I found
working with it was too complex.  This crate is much simpler by comparison,
at the expense of not catching as many bugs.  The design is inspired by
Haskell's HTML combinator libraries such as [blaze-html] and [lucid].

[lucid]: https://hackage.haskell.org/package/lucid
[blaze-html]: https://hackage.haskell.org/package/blaze-html
[askama]: https://docs.rs/askama
[typed-html]: https://docs.rs/typed-html/
