pub fn shorten<T>(string: T) -> String
where
    T: AsRef<str>,
{
    let string = string.as_ref();
    if string.len() < 50 {
        return string.into();
    }

    let mut shortened = String::from(&string[0..47]);
    shortened.push_str("...");
    shortened
}
