mod languages;
mod units;
mod responses;
mod onecall;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::onecall::Query;
    use crate::units::Units;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}




// struct Query {
//
// }

