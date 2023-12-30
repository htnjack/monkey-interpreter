use monkey::repl::Repl;

fn main() {
    let input = std::io::stdin();
    let repl = Repl::new(input);
    repl.run();
}
