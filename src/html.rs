use crate::Node;
use std::borrow::Cow;
use std::fmt::Write;

impl<'a> Node<'a> {
    /// Defines a comment
    pub fn comment<'b>(&'b mut self) -> Node<'b> {
        self.child("<!--", Cow::Borrowed("-->"))
    }

    /// Defines the document type
    pub fn doctype<'b>(&'b mut self) {
        write!(self, "<!DOCTYPE>").unwrap();
    }

    /// Defines a hyperlink
    pub fn a<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<a {}>", attrs), Cow::Borrowed("</a>"))
    }

    /// Defines an abbreviation or an acronym
    pub fn abbr<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<abbr {}>", attrs), Cow::Borrowed("</abbr>"))
    }

    /// Defines contact information for the author/owner of a document
    pub fn address<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<address {}>", attrs), Cow::Borrowed("</address>"))
    }

    /// Defines an area inside an image map
    pub fn area<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<area {}>", attrs), Cow::Borrowed("</area>"))
    }

    /// Defines an article
    pub fn article<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<article {}>", attrs), Cow::Borrowed("</article>"))
    }

    /// Defines content aside from the page content
    pub fn aside<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<aside {}>", attrs), Cow::Borrowed("</aside>"))
    }

    /// Defines embedded sound content
    pub fn audio<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<audio {}>", attrs), Cow::Borrowed("</audio>"))
    }

    /// Defines bold text
    pub fn b<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<b {}>", attrs), Cow::Borrowed("</b>"))
    }

    /// Specifies the base URL/target for all relative URLs in a document
    pub fn base<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<base {}>", attrs), Cow::Borrowed("</base>"))
    }

    /// Isolates a part of text that might be formatted in a different direction from other text outside it
    pub fn bdi<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<bdi {}>", attrs), Cow::Borrowed("</bdi>"))
    }

    /// Overrides the current text direction
    pub fn bdo<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<bdo {}>", attrs), Cow::Borrowed("</bdo>"))
    }

    /// Defines a section that is quoted from another source
    pub fn blockquote<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(
            &format!("<blockquote {}>", attrs),
            Cow::Borrowed("</blockquote>"),
        )
    }

    /// Defines the document's body
    pub fn body<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<body {}>", attrs), Cow::Borrowed("</body>"))
    }

    /// Defines a single line break
    pub fn br<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<br {}>", attrs), Cow::Borrowed("</br>"))
    }

    /// Defines a clickable button
    pub fn button<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<button {}>", attrs), Cow::Borrowed("</button>"))
    }

    /// Used to draw graphics, on the fly, via scripting (usually JavaScript)
    pub fn canvas<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<canvas {}>", attrs), Cow::Borrowed("</canvas>"))
    }

    /// Defines a table caption
    pub fn caption<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<caption {}>", attrs), Cow::Borrowed("</caption>"))
    }

    /// Defines the title of a work
    pub fn cite<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<cite {}>", attrs), Cow::Borrowed("</cite>"))
    }

    /// Defines a piece of computer code
    pub fn code<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<code {}>", attrs), Cow::Borrowed("</code>"))
    }

    /// Specifies column properties for each column within a <colgroup> element
    pub fn col<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<col {}>", attrs), Cow::Borrowed("</col>"))
    }

    /// Specifies a group of one or more columns in a table for formatting
    pub fn colgroup<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(
            &format!("<colgroup {}>", attrs),
            Cow::Borrowed("</colgroup>"),
        )
    }

    /// Adds a machine-readable translation of a given content
    pub fn data<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<data {}>", attrs), Cow::Borrowed("</data>"))
    }

    /// Specifies a list of pre-defined options for input controls
    pub fn datalist<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(
            &format!("<datalist {}>", attrs),
            Cow::Borrowed("</datalist>"),
        )
    }

    /// Defines a description/value of a term in a description list
    pub fn dd<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<dd {}>", attrs), Cow::Borrowed("</dd>"))
    }

    /// Defines text that has been deleted from a document
    pub fn del<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<del {}>", attrs), Cow::Borrowed("</del>"))
    }

    /// Defines additional details that the user can view or hide
    pub fn details<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<details {}>", attrs), Cow::Borrowed("</details>"))
    }

    /// Specifies a term that is going to be defined within the content
    pub fn dfn<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<dfn {}>", attrs), Cow::Borrowed("</dfn>"))
    }

    /// Defines a dialog box or window
    pub fn dialog<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<dialog {}>", attrs), Cow::Borrowed("</dialog>"))
    }

    /// Defines a section in a document
    pub fn div<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<div {}>", attrs), Cow::Borrowed("</div>"))
    }

    /// Defines a description list
    pub fn dl<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<dl {}>", attrs), Cow::Borrowed("</dl>"))
    }

    /// Defines a term/name in a description list
    pub fn dt<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<dt {}>", attrs), Cow::Borrowed("</dt>"))
    }

    /// Defines emphasized text
    pub fn em<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<em {}>", attrs), Cow::Borrowed("</em>"))
    }

    /// Defines a container for an external application
    pub fn embed<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<embed {}>", attrs), Cow::Borrowed("</embed>"))
    }

    /// Groups related elements in a form
    pub fn fieldset<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(
            &format!("<fieldset {}>", attrs),
            Cow::Borrowed("</fieldset>"),
        )
    }

    /// Defines a caption for a <figure> element
    pub fn figcaption<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(
            &format!("<figcaption {}>", attrs),
            Cow::Borrowed("</figcaption>"),
        )
    }

    /// Specifies self-contained content
    pub fn figure<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<figure {}>", attrs), Cow::Borrowed("</figure>"))
    }

    /// Defines a footer for a document or section
    pub fn footer<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<footer {}>", attrs), Cow::Borrowed("</footer>"))
    }

    /// Defines an HTML form for user input
    pub fn form<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<form {}>", attrs), Cow::Borrowed("</form>"))
    }

    /// Defines HTML headings
    pub fn h1<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<h1 {}>", attrs), Cow::Borrowed("</h1>"))
    }

    /// Defines HTML headings
    pub fn h2<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<h2 {}>", attrs), Cow::Borrowed("</h2>"))
    }

    /// Defines HTML headings
    pub fn h3<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<h3 {}>", attrs), Cow::Borrowed("</h3>"))
    }

    /// Defines HTML headings
    pub fn h4<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<h4 {}>", attrs), Cow::Borrowed("</h4>"))
    }

    /// Defines HTML headings
    pub fn h5<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<h5 {}>", attrs), Cow::Borrowed("</h5>"))
    }

    /// Defines HTML headings
    pub fn h6<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<h6 {}>", attrs), Cow::Borrowed("</h6>"))
    }

    /// Contains metadata/information for the document
    pub fn head<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<head {}>", attrs), Cow::Borrowed("</head>"))
    }

    /// Defines a header for a document or section
    pub fn header<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<header {}>", attrs), Cow::Borrowed("</header>"))
    }

    /// Defines a thematic change in the content
    pub fn hr<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<hr {}>", attrs), Cow::Borrowed("</hr>"))
    }

    /// Defines the root of an HTML document
    pub fn html<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<html {}>", attrs), Cow::Borrowed("</html>"))
    }

    /// Defines a part of text in an alternate voice or mood
    pub fn i<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<i {}>", attrs), Cow::Borrowed("</i>"))
    }

    /// Defines an inline frame
    pub fn iframe<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<iframe {}>", attrs), Cow::Borrowed("</iframe>"))
    }

    /// Defines an image
    pub fn img<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<img {}>", attrs), Cow::Borrowed("</img>"))
    }

    /// Defines an input control
    pub fn input<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<input {}>", attrs), Cow::Borrowed("</input>"))
    }

    /// Defines a text that has been inserted into a document
    pub fn ins<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<ins {}>", attrs), Cow::Borrowed("</ins>"))
    }

    /// Defines keyboard input
    pub fn kbd<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<kbd {}>", attrs), Cow::Borrowed("</kbd>"))
    }

    /// Defines a label for an <input> element
    pub fn label<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<label {}>", attrs), Cow::Borrowed("</label>"))
    }

    /// Defines a caption for a <fieldset> element
    pub fn legend<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<legend {}>", attrs), Cow::Borrowed("</legend>"))
    }

    /// Defines a list item
    pub fn li<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<li {}>", attrs), Cow::Borrowed("</li>"))
    }

    /// Defines the relationship between a document and an external resource (most used to link to style sheets)
    pub fn link<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<link {}>", attrs), Cow::Borrowed("</link>"))
    }

    /// Specifies the main content of a document
    pub fn main<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<main {}>", attrs), Cow::Borrowed("</main>"))
    }

    /// Defines an image map
    pub fn map<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<map {}>", attrs), Cow::Borrowed("</map>"))
    }

    /// Defines marked/highlighted text
    pub fn mark<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<mark {}>", attrs), Cow::Borrowed("</mark>"))
    }

    /// Defines metadata about an HTML document
    pub fn meta<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<meta {}>", attrs), Cow::Borrowed("</meta>"))
    }

    /// Defines a scalar measurement within a known range (a gauge)
    pub fn meter<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<meter {}>", attrs), Cow::Borrowed("</meter>"))
    }

    /// Defines navigation links
    pub fn nav<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<nav {}>", attrs), Cow::Borrowed("</nav>"))
    }

    /// Defines an alternate content for users that do not support client-side scripts
    pub fn noscript<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(
            &format!("<noscript {}>", attrs),
            Cow::Borrowed("</noscript>"),
        )
    }

    /// Defines a container for an external application
    pub fn object<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<object {}>", attrs), Cow::Borrowed("</object>"))
    }

    /// Defines an ordered list
    pub fn ol<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<ol {}>", attrs), Cow::Borrowed("</ol>"))
    }

    /// Defines a group of related options in a drop-down list
    pub fn optgroup<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(
            &format!("<optgroup {}>", attrs),
            Cow::Borrowed("</optgroup>"),
        )
    }

    /// Defines an option in a drop-down list
    pub fn option<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<option {}>", attrs), Cow::Borrowed("</option>"))
    }

    /// Defines the result of a calculation
    pub fn output<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<output {}>", attrs), Cow::Borrowed("</output>"))
    }

    /// Defines a paragraph
    pub fn p<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<p {}>", attrs), Cow::Borrowed("</p>"))
    }

    /// Defines a parameter for an object
    pub fn param<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<param {}>", attrs), Cow::Borrowed("</param>"))
    }

    /// Defines a container for multiple image resources
    pub fn picture<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<picture {}>", attrs), Cow::Borrowed("</picture>"))
    }

    /// Defines preformatted text
    pub fn pre<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<pre {}>", attrs), Cow::Borrowed("</pre>"))
    }

    /// Represents the progress of a task
    pub fn progress<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(
            &format!("<progress {}>", attrs),
            Cow::Borrowed("</progress>"),
        )
    }

    /// Defines a short quotation
    pub fn q<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<q {}>", attrs), Cow::Borrowed("</q>"))
    }

    /// Defines what to show in browsers that do not support ruby annotations
    pub fn rp<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<rp {}>", attrs), Cow::Borrowed("</rp>"))
    }

    /// Defines an explanation/pronunciation of characters (for East Asian typography)
    pub fn rt<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<rt {}>", attrs), Cow::Borrowed("</rt>"))
    }

    /// Defines a ruby annotation (for East Asian typography)
    pub fn ruby<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<ruby {}>", attrs), Cow::Borrowed("</ruby>"))
    }

    /// Defines text that is no longer correct
    pub fn s<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<s {}>", attrs), Cow::Borrowed("</s>"))
    }

    /// Defines sample output from a computer program
    pub fn samp<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<samp {}>", attrs), Cow::Borrowed("</samp>"))
    }

    /// Defines a client-side script
    pub fn script<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<script {}>", attrs), Cow::Borrowed("</script>"))
    }

    /// Defines a section in a document
    pub fn section<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<section {}>", attrs), Cow::Borrowed("</section>"))
    }

    /// Defines a drop-down list
    pub fn select<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<select {}>", attrs), Cow::Borrowed("</select>"))
    }

    /// Defines smaller text
    pub fn small<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<small {}>", attrs), Cow::Borrowed("</small>"))
    }

    /// Defines multiple media resources for media elements (<video> and <audio>)
    pub fn source<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<source {}>", attrs), Cow::Borrowed("</source>"))
    }

    /// Defines a section in a document
    pub fn span<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<span {}>", attrs), Cow::Borrowed("</span>"))
    }

    /// Defines important text
    pub fn strong<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<strong {}>", attrs), Cow::Borrowed("</strong>"))
    }

    /// Defines style information for a document
    pub fn style<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<style {}>", attrs), Cow::Borrowed("</style>"))
    }

    /// Defines subscripted text
    pub fn sub<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<sub {}>", attrs), Cow::Borrowed("</sub>"))
    }

    /// Defines a visible heading for a <details> element
    pub fn summary<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<summary {}>", attrs), Cow::Borrowed("</summary>"))
    }

    /// Defines superscripted text
    pub fn sup<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<sup {}>", attrs), Cow::Borrowed("</sup>"))
    }

    /// Defines a container for SVG graphics
    pub fn svg<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<svg {}>", attrs), Cow::Borrowed("</svg>"))
    }

    /// Defines a table
    pub fn table<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<table {}>", attrs), Cow::Borrowed("</table>"))
    }

    /// Groups the body content in a table
    pub fn tbody<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<tbody {}>", attrs), Cow::Borrowed("</tbody>"))
    }

    /// Defines a cell in a table
    pub fn td<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<td {}>", attrs), Cow::Borrowed("</td>"))
    }

    /// Defines a container for content that should be hidden when the page loads
    pub fn template<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(
            &format!("<template {}>", attrs),
            Cow::Borrowed("</template>"),
        )
    }

    /// Defines a multiline input control (text area)
    pub fn textarea<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(
            &format!("<textarea {}>", attrs),
            Cow::Borrowed("</textarea>"),
        )
    }

    /// Groups the footer content in a table
    pub fn tfoot<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<tfoot {}>", attrs), Cow::Borrowed("</tfoot>"))
    }

    /// Defines a header cell in a table
    pub fn th<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<th {}>", attrs), Cow::Borrowed("</th>"))
    }

    /// Groups the header content in a table
    pub fn thead<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<thead {}>", attrs), Cow::Borrowed("</thead>"))
    }

    /// Defines a specific time (or datetime)
    pub fn time<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<time {}>", attrs), Cow::Borrowed("</time>"))
    }

    /// Defines a title for the document
    pub fn title<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<title {}>", attrs), Cow::Borrowed("</title>"))
    }

    /// Defines a row in a table
    pub fn tr<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<tr {}>", attrs), Cow::Borrowed("</tr>"))
    }

    /// Defines text tracks for media elements (<video> and <audio>)
    pub fn track<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<track {}>", attrs), Cow::Borrowed("</track>"))
    }

    /// Defines some text that is unarticulated and styled differently from normal text
    pub fn u<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<u {}>", attrs), Cow::Borrowed("</u>"))
    }

    /// Defines an unordered list
    pub fn ul<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<ul {}>", attrs), Cow::Borrowed("</ul>"))
    }

    /// Defines a variable
    pub fn var<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<var {}>", attrs), Cow::Borrowed("</var>"))
    }

    /// Defines embedded video content
    pub fn video<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<video {}>", attrs), Cow::Borrowed("</video>"))
    }

    /// Defines a possible line-break
    pub fn wbr<'b>(&'b mut self, attrs: &str) -> Node<'b> {
        self.child(&format!("<wbr {}>", attrs), Cow::Borrowed("</wbr>"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Root;

    #[test]
    pub fn test() {
        let mut root = Root::new();
        root.doctype();
        let mut html = root.html("");
        write!(html.head("").title(""), "Website!").unwrap();
        let mut body = html.body("");
        write!(body.h1(""), "It's a website!").unwrap();
        let mut list = body.ul("");
        for i in 0..5 {
            write!(
                list.li("").a(&format!("href='/page_{}.html'", i)),
                "Page {}",
                i
            )
            .unwrap()
        }

        let expected = r#"
<!DOCTYPE>
<html >
<head ><title >Website!</title></head>
<body >
<h1 >It's a website!</h1>
<ul >
<li ><a href='/page_0.html'>Page 0</a></li>
<li ><a href='/page_1.html'>Page 1</a></li>
<li ><a href='/page_2.html'>Page 2</a></li>
<li ><a href='/page_3.html'>Page 3</a></li>
<li ><a href='/page_4.html'>Page 4</a></li>
</ul>
</body>
</html>
"#;
        assert_eq!(root.build(), expected.lines().collect::<Vec<_>>().join(""));
    }
}
