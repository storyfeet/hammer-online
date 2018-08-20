use sqlite;

use self::SCServerErr::*;
use bcrypt::BcryptError;


#[derive(Debug)]
pub enum SCServerErr {
    DbErr(sqlite::Error),
    HashErr(BcryptError),
}

impl From<sqlite::Error> for SCServerErr {
    fn from(e:sqlite::Error)->Self{
        DbErr(e)
    }
}

impl From<BcryptError> for SCServerErr{
    fn from(e:BcryptError)->Self{
        HashErr(e)
    }
}
