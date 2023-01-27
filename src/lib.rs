mod languages;
mod query;
mod units;
mod responses;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::query::Query;
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

