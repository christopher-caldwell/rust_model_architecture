pub trait IdentGeneratorPort: Send + Sync {
    fn gen(&self) -> String;
}
