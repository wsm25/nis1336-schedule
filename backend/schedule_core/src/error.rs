pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UserNotExist,
    UserExists,
    IncorrectPassword,
    SledError(sled::Error),
    BrokenDB,
    BinaryError(bincode::Error),
    TaskNotFound(u64),
    TaskExists(u64),
}

impl From<sled::Error> for Error {
    fn from(value: sled::Error) -> Self 
        {Self::SledError(value)}}

impl From<bincode::Error> for Error {
    fn from(value: bincode::Error) -> Self 
        {Self::BinaryError(value)}}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) 
        -> std::fmt::Result 
    {
        use Error::*;
        match self{
            UserExists=>write!(f, "user already exists"),
            UserNotExist=>write!(f, "user does not exist"),
            TaskNotFound(id)=>write!(f, "task id {id} does not exist"),
            TaskExists(id)=>write!(f, "task {id} already exists"),
            IncorrectPassword=>write!(f, "password incorrect"),
            _ => write!(f, "internal error `{self:?}`"),
        }
    }
}

impl std::error::Error for Error {}

