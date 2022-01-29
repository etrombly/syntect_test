use syntect::dumps::dump_to_file;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSetBuilder;

fn main() {
    let mut syntax_set_builder = SyntaxSetBuilder::new();
    syntax_set_builder
        .add_from_folder("./assets", true)
        .unwrap();
    let syntax_set = syntax_set_builder.build();
    dump_to_file(&syntax_set, "./assets/ss.bin").unwrap();
    let ts = ThemeSet::load_from_folder("./assets").unwrap();
    dump_to_file(&ts, "./assets/ts.bin").unwrap();
}
