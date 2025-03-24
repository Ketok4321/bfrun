use bfrun::interpreter::*;
use bfrun::parser::*;

const CODE: &str = "
++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.
";

fn main() {
    let code = parse(CODE);

    run(8, &code);
}
