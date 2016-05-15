extern crate gcc;

fn main() {
    gcc::compile_library("liblevenshtein.a", &["src/levenshtein.c/levenshtein.c"]);
}
