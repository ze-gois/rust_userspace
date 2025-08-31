pub trait Labeled<Origin> {
    fn description(&self) -> &str;
    fn acronym(&self) -> &str;
}
