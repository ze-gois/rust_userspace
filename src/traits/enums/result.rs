pub trait Result {
    type ResultType;

    // fn from_no(no: Self::ResultType) -> Self;

    fn acronym(&self) -> &'static str;

    fn description(&self) -> &'static str;
}
