use crate::{Node, Void};
use std::borrow::Cow;

/// Helper methods for generating HTML5 documents.
pub trait Html5 {
    /// Defines the document type
    fn doctype(&mut self);

    /// Defines a hyperlink
    fn a(&mut self) -> Node;

    /// Defines an abbreviation or an acronym
    fn abbr(&mut self) -> Node;

    /// Defines contact information for the author/owner of a document
    fn address(&mut self) -> Node;

    /// Defines an area inside an image map
    fn area(&mut self) -> Void;

    /// Defines an article
    fn article(&mut self) -> Node;

    /// Defines content aside from the page content
    fn aside(&mut self) -> Node;

    /// Defines embedded sound content
    fn audio(&mut self) -> Node;

    /// Defines bold text
    fn b(&mut self) -> Node;

    /// Specifies the base URL/target for all relative URLs in a document
    fn base(&mut self) -> Void;

    /// Isolates a part of text that might be formatted in a different direction from other text outside it
    fn bdi(&mut self) -> Node;

    /// Overrides the current text direction
    fn bdo(&mut self) -> Node;

    /// Defines a section that is quoted from another source
    fn blockquote(&mut self) -> Node;

    /// Defines the document's body
    fn body(&mut self) -> Node;

    /// Defines a single line break
    fn br(&mut self) -> Void;

    /// Defines a clickable button
    fn button(&mut self) -> Node;

    /// Used to draw graphics, on the fly, via scripting (usually JavaScript)
    fn canvas(&mut self) -> Node;

    /// Defines a table caption
    fn caption(&mut self) -> Node;

    /// Defines the title of a work
    fn cite(&mut self) -> Node;

    /// Defines a piece of computer code
    fn code(&mut self) -> Node;

    /// Specifies column properties for each column within a <colgroup> element
    fn col(&mut self) -> Void;

    /// Specifies a group of one or more columns in a table for formatting
    fn colgroup(&mut self) -> Node;

    /// Adds a machine-readable translation of a given content
    fn data(&mut self) -> Node;

    /// Specifies a list of pre-defined options for input controls
    fn datalist(&mut self) -> Node;

    /// Defines a description/value of a term in a description list
    fn dd(&mut self) -> Node;

    /// Defines text that has been deleted from a document
    fn del(&mut self) -> Node;

    /// Defines additional details that the user can view or hide
    fn details(&mut self) -> Node;

    /// Specifies a term that is going to be defined within the content
    fn dfn(&mut self) -> Node;

    /// Defines a dialog box or window
    fn dialog(&mut self) -> Node;

    /// Defines a section in a document
    fn div(&mut self) -> Node;

    /// Defines a description list
    fn dl(&mut self) -> Node;

    /// Defines a term/name in a description list
    fn dt(&mut self) -> Node;

    /// Defines emphasized text
    fn em(&mut self) -> Node;

    /// Defines a container for an external application
    fn embed(&mut self) -> Void;

    /// Groups related elements in a form
    fn fieldset(&mut self) -> Node;

    /// Defines a caption for a `<figure>` element
    fn figcaption(&mut self) -> Node;

    /// Specifies self-contained content
    fn figure(&mut self) -> Node;

    /// Defines a footer for a document or section
    fn footer(&mut self) -> Node;

    /// Defines an HTML form for user input
    fn form(&mut self) -> Node;

    /// Defines HTML headings
    fn h1(&mut self) -> Node;

    /// Defines HTML headings
    fn h2(&mut self) -> Node;

    /// Defines HTML headings
    fn h3(&mut self) -> Node;

    /// Defines HTML headings
    fn h4(&mut self) -> Node;

    /// Defines HTML headings
    fn h5(&mut self) -> Node;

    /// Defines HTML headings
    fn h6(&mut self) -> Node;

    /// Contains metadata/information for the document
    fn head(&mut self) -> Node;

    /// Defines a header for a document or section
    fn header(&mut self) -> Node;

    /// Defines a thematic change in the content
    fn hr(&mut self) -> Void;

    /// Defines the root of an HTML document
    fn html(&mut self) -> Node;

    /// Defines a part of text in an alternate voice or mood
    fn i(&mut self) -> Node;

    /// Defines an inline frame
    fn iframe(&mut self) -> Node;

    /// Defines an image
    fn img(&mut self) -> Void;

    /// Defines an input control
    fn input(&mut self) -> Void;

    /// Defines a text that has been inserted into a document
    fn ins(&mut self) -> Node;

    /// Defines keyboard input
    fn kbd(&mut self) -> Node;

    /// Defines a label for an `<input>` element
    fn label(&mut self) -> Node;

    /// Defines a caption for a `<fieldset>` element
    fn legend(&mut self) -> Node;

    /// Defines a list item
    fn li(&mut self) -> Node;

    /// Defines the relationship between a document and an external resource (most used to link to style sheets)
    fn link(&mut self) -> Void;

    /// Specifies the main content of a document
    fn main(&mut self) -> Node;

    /// Defines an image map
    fn map(&mut self) -> Node;

    /// Defines marked/highlighted text
    fn mark(&mut self) -> Node;

    /// Defines metadata about an HTML document
    fn meta(&mut self) -> Void;

    /// Defines a scalar measurement within a known range (a gauge)
    fn meter(&mut self) -> Node;

    /// Defines navigation links
    fn nav(&mut self) -> Node;

    /// Defines an alternate content for users that do not support client-side scripts
    fn noscript(&mut self) -> Node;

    /// Defines a container for an external application
    fn object(&mut self) -> Node;

    /// Defines an ordered list
    fn ol(&mut self) -> Node;

    /// Defines a group of related options in a drop-down list
    fn optgroup(&mut self) -> Node;

    /// Defines an option in a drop-down list
    fn option(&mut self) -> Node;

    /// Defines the result of a calculation
    fn output(&mut self) -> Node;

    /// Defines a paragraph
    fn p(&mut self) -> Node;

    /// Defines a parameter for an object
    fn param(&mut self) -> Void;

    /// Defines a container for multiple image resources
    fn picture(&mut self) -> Node;

    /// Defines preformatted text
    fn pre(&mut self) -> Node;

    /// Represents the progress of a task
    fn progress(&mut self) -> Node;

    /// Defines a short quotation
    fn q(&mut self) -> Node;

    /// Defines what to show in browsers that do not support ruby annotations
    fn rp(&mut self) -> Node;

    /// Defines an explanation/pronunciation of characters (for East Asian typography)
    fn rt(&mut self) -> Node;

    /// Defines a ruby annotation (for East Asian typography)
    fn ruby(&mut self) -> Node;

    /// Defines text that is no longer correct
    fn s(&mut self) -> Node;

    /// Defines sample output from a computer program
    fn samp(&mut self) -> Node;

    /// Defines a client-side script
    fn script(&mut self) -> Node;

    /// Defines a section in a document
    fn section(&mut self) -> Node;

    /// Defines a drop-down list
    fn select(&mut self) -> Node;

    /// Defines smaller text
    fn small(&mut self) -> Node;

    /// Defines multiple media resources for media elements (`<video>` and `<audio>`)
    fn source(&mut self) -> Void;

    /// Defines a section in a document
    fn span(&mut self) -> Node;

    /// Defines important text
    fn strong(&mut self) -> Node;

    /// Defines style information for a document
    fn style(&mut self) -> Node;

    /// Defines subscripted text
    fn sub(&mut self) -> Node;

    /// Defines a visible heading for a `<details>` element
    fn summary(&mut self) -> Node;

    /// Defines superscripted text
    fn sup(&mut self) -> Node;

    /// Defines a container for SVG graphics
    fn svg(&mut self) -> Node;

    /// Defines a table
    fn table(&mut self) -> Node;

    /// Groups the body content in a table
    fn tbody(&mut self) -> Node;

    /// Defines a cell in a table
    fn td(&mut self) -> Node;

    /// Defines a container for content that should be hidden when the page loads
    fn template(&mut self) -> Node;

    /// Defines a multiline input control (text area)
    fn textarea(&mut self) -> Node;

    /// Groups the footer content in a table
    fn tfoot(&mut self) -> Node;

    /// Defines a header cell in a table
    fn th(&mut self) -> Node;

    /// Groups the header content in a table
    fn thead(&mut self) -> Node;

    /// Defines a specific time (or datetime)
    fn time(&mut self) -> Node;

    /// Defines a title for the document
    fn title(&mut self) -> Node;

    /// Defines a row in a table
    fn tr(&mut self) -> Node;

    /// Defines text tracks for media elements (`<video>` and `<audio>`)
    fn track(&mut self) -> Void;

    /// Defines some text that is unarticulated and styled differently from normal text
    fn u(&mut self) -> Node;

    /// Defines an unordered list
    fn ul(&mut self) -> Node;

    /// Defines a variable
    fn var(&mut self) -> Node;

    /// Defines embedded video content
    fn video(&mut self) -> Node;

    /// Defines a possible line-break
    fn wbr(&mut self) -> Void;
}

impl<'a> Html5 for Node<'a> {
    /// Defines the document type
    fn doctype(&mut self) {
        self.void_child(Cow::Borrowed("!DOCTYPE")).attr("html");
    }

    /// Defines a hyperlink
    fn a(&mut self) -> Node {
        self.child(Cow::Borrowed("a"))
    }

    /// Defines an abbreviation or an acronym
    fn abbr(&mut self) -> Node {
        self.child(Cow::Borrowed("abbr"))
    }

    /// Defines contact information for the author/owner of a document
    fn address(&mut self) -> Node {
        self.child(Cow::Borrowed("address"))
    }

    /// Defines an area inside an image map
    fn area(&mut self) -> Void {
        self.void_child(Cow::Borrowed("area"))
    }

    /// Defines an article
    fn article(&mut self) -> Node {
        self.child(Cow::Borrowed("article"))
    }

    /// Defines content aside from the page content
    fn aside(&mut self) -> Node {
        self.child(Cow::Borrowed("aside"))
    }

    /// Defines embedded sound content
    fn audio(&mut self) -> Node {
        self.child(Cow::Borrowed("audio"))
    }

    /// Defines bold text
    fn b(&mut self) -> Node {
        self.child(Cow::Borrowed("b"))
    }

    /// Specifies the base URL/target for all relative URLs in a document
    fn base(&mut self) -> Void {
        self.void_child(Cow::Borrowed("base"))
    }

    /// Isolates a part of text that might be formatted in a different direction from other text outside it
    fn bdi(&mut self) -> Node {
        self.child(Cow::Borrowed("bdi"))
    }

    /// Overrides the current text direction
    fn bdo(&mut self) -> Node {
        self.child(Cow::Borrowed("bdo"))
    }

    /// Defines a section that is quoted from another source
    fn blockquote(&mut self) -> Node {
        self.child(Cow::Borrowed("blockquote"))
    }

    /// Defines the document's body
    fn body(&mut self) -> Node {
        self.child(Cow::Borrowed("body"))
    }

    /// Defines a single line break
    fn br(&mut self) -> Void {
        self.void_child(Cow::Borrowed("br"))
    }

    /// Defines a clickable button
    fn button(&mut self) -> Node {
        self.child(Cow::Borrowed("button"))
    }

    /// Used to draw graphics, on the fly, via scripting (usually JavaScript)
    fn canvas(&mut self) -> Node {
        self.child(Cow::Borrowed("canvas"))
    }

    /// Defines a table caption
    fn caption(&mut self) -> Node {
        self.child(Cow::Borrowed("caption"))
    }

    /// Defines the title of a work
    fn cite(&mut self) -> Node {
        self.child(Cow::Borrowed("cite"))
    }

    /// Defines a piece of computer code
    fn code(&mut self) -> Node {
        self.child(Cow::Borrowed("code"))
    }

    /// Specifies column properties for each column within a <colgroup> element
    fn col(&mut self) -> Void {
        self.void_child(Cow::Borrowed("col"))
    }

    /// Specifies a group of one or more columns in a table for formatting
    fn colgroup(&mut self) -> Node {
        self.child(Cow::Borrowed("colgroup"))
    }

    /// Adds a machine-readable translation of a given content
    fn data(&mut self) -> Node {
        self.child(Cow::Borrowed("data"))
    }

    /// Specifies a list of pre-defined options for input controls
    fn datalist(&mut self) -> Node {
        self.child(Cow::Borrowed("datalist"))
    }

    /// Defines a description/value of a term in a description list
    fn dd(&mut self) -> Node {
        self.child(Cow::Borrowed("dd"))
    }

    /// Defines text that has been deleted from a document
    fn del(&mut self) -> Node {
        self.child(Cow::Borrowed("del"))
    }

    /// Defines additional details that the user can view or hide
    fn details(&mut self) -> Node {
        self.child(Cow::Borrowed("details"))
    }

    /// Specifies a term that is going to be defined within the content
    fn dfn(&mut self) -> Node {
        self.child(Cow::Borrowed("dfn"))
    }

    /// Defines a dialog box or window
    fn dialog(&mut self) -> Node {
        self.child(Cow::Borrowed("dialog"))
    }

    /// Defines a section in a document
    fn div(&mut self) -> Node {
        self.child(Cow::Borrowed("div"))
    }

    /// Defines a description list
    fn dl(&mut self) -> Node {
        self.child(Cow::Borrowed("dl"))
    }

    /// Defines a term/name in a description list
    fn dt(&mut self) -> Node {
        self.child(Cow::Borrowed("dt"))
    }

    /// Defines emphasized text
    fn em(&mut self) -> Node {
        self.child(Cow::Borrowed("em"))
    }

    /// Defines a container for an external application
    fn embed(&mut self) -> Void {
        self.void_child(Cow::Borrowed("embed"))
    }

    /// Groups related elements in a form
    fn fieldset(&mut self) -> Node {
        self.child(Cow::Borrowed("fieldset"))
    }

    /// Defines a caption for a <figure> element
    fn figcaption(&mut self) -> Node {
        self.child(Cow::Borrowed("figcaption"))
    }

    /// Specifies self-contained content
    fn figure(&mut self) -> Node {
        self.child(Cow::Borrowed("figure"))
    }

    /// Defines a footer for a document or section
    fn footer(&mut self) -> Node {
        self.child(Cow::Borrowed("footer"))
    }

    /// Defines an HTML form for user input
    fn form(&mut self) -> Node {
        self.child(Cow::Borrowed("form"))
    }

    /// Defines HTML headings
    fn h1(&mut self) -> Node {
        self.child(Cow::Borrowed("h1"))
    }

    /// Defines HTML headings
    fn h2(&mut self) -> Node {
        self.child(Cow::Borrowed("h2"))
    }

    /// Defines HTML headings
    fn h3(&mut self) -> Node {
        self.child(Cow::Borrowed("h3"))
    }

    /// Defines HTML headings
    fn h4(&mut self) -> Node {
        self.child(Cow::Borrowed("h4"))
    }

    /// Defines HTML headings
    fn h5(&mut self) -> Node {
        self.child(Cow::Borrowed("h5"))
    }

    /// Defines HTML headings
    fn h6(&mut self) -> Node {
        self.child(Cow::Borrowed("h6"))
    }

    /// Contains metadata/information for the document
    fn head(&mut self) -> Node {
        self.child(Cow::Borrowed("head"))
    }

    /// Defines a header for a document or section
    fn header(&mut self) -> Node {
        self.child(Cow::Borrowed("header"))
    }

    /// Defines a thematic change in the content
    fn hr(&mut self) -> Void {
        self.void_child(Cow::Borrowed("hr"))
    }

    /// Defines the root of an HTML document
    fn html(&mut self) -> Node {
        self.child(Cow::Borrowed("html"))
    }

    /// Defines a part of text in an alternate voice or mood
    fn i(&mut self) -> Node {
        self.child(Cow::Borrowed("i"))
    }

    /// Defines an inline frame
    fn iframe(&mut self) -> Node {
        self.child(Cow::Borrowed("iframe"))
    }

    /// Defines an image
    fn img(&mut self) -> Void {
        self.void_child(Cow::Borrowed("img"))
    }

    /// Defines an input control
    fn input(&mut self) -> Void {
        self.void_child(Cow::Borrowed("input"))
    }

    /// Defines a text that has been inserted into a document
    fn ins(&mut self) -> Node {
        self.child(Cow::Borrowed("ins"))
    }

    /// Defines keyboard input
    fn kbd(&mut self) -> Node {
        self.child(Cow::Borrowed("kbd"))
    }

    /// Defines a label for an <input> element
    fn label(&mut self) -> Node {
        self.child(Cow::Borrowed("label"))
    }

    /// Defines a caption for a <fieldset> element
    fn legend(&mut self) -> Node {
        self.child(Cow::Borrowed("legend"))
    }

    /// Defines a list item
    fn li(&mut self) -> Node {
        self.child(Cow::Borrowed("li"))
    }

    /// Defines the relationship between a document and an external resource (most used to link to style sheets)
    fn link(&mut self) -> Void {
        self.void_child(Cow::Borrowed("link"))
    }

    /// Specifies the main content of a document
    fn main(&mut self) -> Node {
        self.child(Cow::Borrowed("main"))
    }

    /// Defines an image map
    fn map(&mut self) -> Node {
        self.child(Cow::Borrowed("map"))
    }

    /// Defines marked/highlighted text
    fn mark(&mut self) -> Node {
        self.child(Cow::Borrowed("mark"))
    }

    /// Defines metadata about an HTML document
    fn meta(&mut self) -> Void {
        self.void_child(Cow::Borrowed("meta"))
    }

    /// Defines a scalar measurement within a known range (a gauge)
    fn meter(&mut self) -> Node {
        self.child(Cow::Borrowed("meter"))
    }

    /// Defines navigation links
    fn nav(&mut self) -> Node {
        self.child(Cow::Borrowed("nav"))
    }

    /// Defines an alternate content for users that do not support client-side scripts
    fn noscript(&mut self) -> Node {
        self.child(Cow::Borrowed("noscript"))
    }

    /// Defines a container for an external application
    fn object(&mut self) -> Node {
        self.child(Cow::Borrowed("object"))
    }

    /// Defines an ordered list
    fn ol(&mut self) -> Node {
        self.child(Cow::Borrowed("ol"))
    }

    /// Defines a group of related options in a drop-down list
    fn optgroup(&mut self) -> Node {
        self.child(Cow::Borrowed("optgroup"))
    }

    /// Defines an option in a drop-down list
    fn option(&mut self) -> Node {
        self.child(Cow::Borrowed("option"))
    }

    /// Defines the result of a calculation
    fn output(&mut self) -> Node {
        self.child(Cow::Borrowed("output"))
    }

    /// Defines a paragraph
    fn p(&mut self) -> Node {
        self.child(Cow::Borrowed("p"))
    }

    /// Defines a parameter for an object
    fn param(&mut self) -> Void {
        self.void_child(Cow::Borrowed("param"))
    }

    /// Defines a container for multiple image resources
    fn picture(&mut self) -> Node {
        self.child(Cow::Borrowed("picture"))
    }

    /// Defines preformatted text
    fn pre(&mut self) -> Node {
        self.child(Cow::Borrowed("pre"))
    }

    /// Represents the progress of a task
    fn progress(&mut self) -> Node {
        self.child(Cow::Borrowed("progress"))
    }

    /// Defines a short quotation
    fn q(&mut self) -> Node {
        self.child(Cow::Borrowed("q"))
    }

    /// Defines what to show in browsers that do not support ruby annotations
    fn rp(&mut self) -> Node {
        self.child(Cow::Borrowed("rp"))
    }

    /// Defines an explanation/pronunciation of characters (for East Asian typography)
    fn rt(&mut self) -> Node {
        self.child(Cow::Borrowed("rt"))
    }

    /// Defines a ruby annotation (for East Asian typography)
    fn ruby(&mut self) -> Node {
        self.child(Cow::Borrowed("ruby"))
    }

    /// Defines text that is no longer correct
    fn s(&mut self) -> Node {
        self.child(Cow::Borrowed("s"))
    }

    /// Defines sample output from a computer program
    fn samp(&mut self) -> Node {
        self.child(Cow::Borrowed("samp"))
    }

    /// Defines a client-side script
    fn script(&mut self) -> Node {
        self.child(Cow::Borrowed("script"))
    }

    /// Defines a section in a document
    fn section(&mut self) -> Node {
        self.child(Cow::Borrowed("section"))
    }

    /// Defines a drop-down list
    fn select(&mut self) -> Node {
        self.child(Cow::Borrowed("select"))
    }

    /// Defines smaller text
    fn small(&mut self) -> Node {
        self.child(Cow::Borrowed("small"))
    }

    /// Defines multiple media resources for media elements (<video> and <audio>)
    fn source(&mut self) -> Void {
        self.void_child(Cow::Borrowed("source"))
    }

    /// Defines a section in a document
    fn span(&mut self) -> Node {
        self.child(Cow::Borrowed("span"))
    }

    /// Defines important text
    fn strong(&mut self) -> Node {
        self.child(Cow::Borrowed("strong"))
    }

    /// Defines style information for a document
    fn style(&mut self) -> Node {
        self.child(Cow::Borrowed("style"))
    }

    /// Defines subscripted text
    fn sub(&mut self) -> Node {
        self.child(Cow::Borrowed("sub"))
    }

    /// Defines a visible heading for a <details> element
    fn summary(&mut self) -> Node {
        self.child(Cow::Borrowed("summary"))
    }

    /// Defines superscripted text
    fn sup(&mut self) -> Node {
        self.child(Cow::Borrowed("sup"))
    }

    /// Defines a container for SVG graphics
    fn svg(&mut self) -> Node {
        self.child(Cow::Borrowed("svg"))
    }

    /// Defines a table
    fn table(&mut self) -> Node {
        self.child(Cow::Borrowed("table"))
    }

    /// Groups the body content in a table
    fn tbody(&mut self) -> Node {
        self.child(Cow::Borrowed("tbody"))
    }

    /// Defines a cell in a table
    fn td(&mut self) -> Node {
        self.child(Cow::Borrowed("td"))
    }

    /// Defines a container for content that should be hidden when the page loads
    fn template(&mut self) -> Node {
        self.child(Cow::Borrowed("template"))
    }

    /// Defines a multiline input control (text area)
    fn textarea(&mut self) -> Node {
        self.child(Cow::Borrowed("textarea"))
    }

    /// Groups the footer content in a table
    fn tfoot(&mut self) -> Node {
        self.child(Cow::Borrowed("tfoot"))
    }

    /// Defines a header cell in a table
    fn th(&mut self) -> Node {
        self.child(Cow::Borrowed("th"))
    }

    /// Groups the header content in a table
    fn thead(&mut self) -> Node {
        self.child(Cow::Borrowed("thead"))
    }

    /// Defines a specific time (or datetime)
    fn time(&mut self) -> Node {
        self.child(Cow::Borrowed("time"))
    }

    /// Defines a title for the document
    fn title(&mut self) -> Node {
        self.child(Cow::Borrowed("title"))
    }

    /// Defines a row in a table
    fn tr(&mut self) -> Node {
        self.child(Cow::Borrowed("tr"))
    }

    /// Defines text tracks for media elements (<video> and <audio>)
    fn track(&mut self) -> Void {
        self.void_child(Cow::Borrowed("track"))
    }

    /// Defines some text that is unarticulated and styled differently from normal text
    fn u(&mut self) -> Node {
        self.child(Cow::Borrowed("u"))
    }

    /// Defines an unordered list
    fn ul(&mut self) -> Node {
        self.child(Cow::Borrowed("ul"))
    }

    /// Defines a variable
    fn var(&mut self) -> Node {
        self.child(Cow::Borrowed("var"))
    }

    /// Defines embedded video content
    fn video(&mut self) -> Node {
        self.child(Cow::Borrowed("video"))
    }

    /// Defines a possible line-break
    fn wbr(&mut self) -> Void {
        self.void_child(Cow::Borrowed("wbr"))
    }
}
