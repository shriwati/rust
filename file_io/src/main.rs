
fn main() {
    let cli = Cli::parse();
    let filename = cli.f;
    let f = FileInfo::new(filename);
    f.read_file(&cli.t.to_ascii_lowercase());


}
