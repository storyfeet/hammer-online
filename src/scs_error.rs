use sqlite;

use self::SCServerErr::*;
use bcrypt::BcryptError;
use std::sync::PoisonError;
use std::num::ParseIntError;
use shoehorn_circle::ScErr;


#[derive(Debug)]
pub enum SCServerErr {
    DbErr(sqlite::Error),
    HashErr(BcryptError),
    NotFound,
    NoUser,
    PasswordFail,
    MutexPoisoned,
    NoCookie,
    ParseErr,
    GameErr(ScErr),
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
impl<T> From<PoisonError<T>> for SCServerErr{
    fn from(_:PoisonError<T>)->Self{
        MutexPoisoned
    }
}

impl From<ParseIntError> for SCServerErr{
    fn from(_:ParseIntError)->Self{
        ParseErr
    }
}

impl From<ScErr> for SCServerErr{
    fn from(e:ScErr)->Self{
        GameErr(e)
    }
}
