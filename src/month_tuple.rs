use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct MonthTuple {
    y: u32,
    m: u32,
}

impl fmt::Display for MonthTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}{:02}", self.y, self.m)
    }
}

#[cfg(test)]
mod tests {

    //todo nextmonth
    //todo previousmonth
    //todo toreadablestring

    #[test]
    fn test_to_string() {
        let tuple = super::MonthTuple { y: 2000, m: 5 };
        assert_eq!(String::from("200005"), tuple.to_string());
    }

    //todo equals
    //todo compareto

}
