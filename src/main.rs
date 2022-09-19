use railwindcss::parse_html;
use std::path::Path;

fn main() {
    parse_html(Path::new("index.html"), Path::new("railwind.css"));
}
