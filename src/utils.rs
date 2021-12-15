pub type Failable<T> = Result<T, String>;
pub type DayResult = Failable<(Failable<String>, Failable<String>)>;
