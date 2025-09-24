pub fn main() -> std::io::Result<()> {
    abc_cli_flags::run_args(std::env::args(), &mut std::io::stdout())
}
