use diesel::QueryResult;
#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    InsertNumError,
    DuplicateData(String),
    WapperError(String),
    NotFound,
    ForeignKeyViolation(String),
}

#[allow(bindings_with_variant_name)]
#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(dead_code)]
pub fn deal_insert_result(r: QueryResult<usize>) -> Result<(), Error> {
    match r {
        Ok(s) => {
            if s == 1 {
                Ok(())
            } else {
                Err(Error::InsertNumError)
            }
        }
        Err(e) => {
            if let diesel::result::Error::DatabaseError(UniqueViolation, _) = e {
                Err(Error::DuplicateData(e.to_string()))
            } else {
                Err(Error::WapperError(e.to_string()))
            }
        }
    }
}

pub fn deal_query_result<T>(r: QueryResult<T>) -> Result<T, Error> {
    match r {
        Ok(u) => Ok(u),
        Err(e) => {
            if let diesel::NotFound = e {
                Err(Error::NotFound)
            } else {
                Err(Error::WapperError(e.to_string()))
            }
        }
    }
}
#[allow(dead_code)]
pub fn deal_update_result(r: QueryResult<usize>) -> Result<(), Error> {
    match r {
        Ok(s) => {
            if s == 1 {
                Ok(())
            } else {
                Err(Error::NotFound)
            }
        }
        Err(e) => {
            if let diesel::NotFound = e {
                Err(Error::NotFound)
            } else {
                Err(Error::WapperError(e.to_string()))
            }
        }
    }
}
