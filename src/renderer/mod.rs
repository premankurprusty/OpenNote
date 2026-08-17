use crate::ast::Document;
mod htmlbuild;
use htmlbuild::html_build;
pub fn render(document: Document) -> String {
    let mut result = String::new();
    let output = html_build(document);
    result.push_str("<html>\n");
    result.push_str("<head>\n");
    result.push_str("    <meta charset=\"utf-8\">\n");
    result.push_str("</head>\n");
    result.push_str("<body>\n");
    result.push_str(&output);
    result.push_str("\n");
    result.push_str("</body>\n");
    result.push_str("</html>\n");
    result
}
