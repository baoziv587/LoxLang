use std::fs::File;
use std::io::prelude::*;

pub fn exec(dist: &str) {
    info!(
        "\
        \n===================\n\
        Start AST Generator\n\
        ===================\n
        "
    )
}

fn gen_ast(out_dit: &str) {
    define_ast(out_dit, "Expr", &vec![
        "Binary   : Expr left, Token operator, Expr right",
        "Grouping : Expr expression",
        "Literal  : Object value",
        "Unary    : Token operator, Expr right",
    ]);
}

fn define_ast(out_dir: &str, base_name: &str, ast_types: &Vec<&str>) {
//    let output_path=
}

fn write(name: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::open("name")?;
    file.write(content.as_bytes())?;
    Ok(())
}
