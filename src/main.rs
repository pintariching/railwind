use std::path::Path;

use railwindcss::parse_html;

fn main() {
    let _ = parse_html(Path::new("index.html"), Path::new("railwind.css"));
}
