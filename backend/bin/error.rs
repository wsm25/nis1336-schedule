#[derive(Debug)]
pub enum Error{
    OK, // default success message
    Core(sccore::Error),
    BadCookie,
    AlreadyLogin,
    InvalidUser,
    PwdTooShort,
    Unauthorized,
}
pub type Result<T> = std::result::Result<T, Error>;

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        use Error::*;
        use sccore::Error::*;
        use axum::http::StatusCode;
        let code = match &self {
            OK => StatusCode::OK,
            Core(e) => match e {
                UserNotExist | TaskNotFound(_) => StatusCode::NOT_FOUND,
                UserExists | TaskExists(_) => StatusCode::FORBIDDEN,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            AlreadyLogin | InvalidUser | PwdTooShort => StatusCode::FORBIDDEN,
            BadCookie  => StatusCode::BAD_REQUEST,
            Unauthorized => StatusCode::UNAUTHORIZED,
            // _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        #[derive(serde::Serialize)]
        struct Resp {message: String}
        (
            code, 
            axum::Json(Resp{
                message: format!("{self}"),
            })
        ).into_response()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            OK => write!(f, "succeed"),
            Core(e) => write!(f, "{e}"),
            BadCookie => write!(f, "bad cookie"),
            AlreadyLogin => write!(f, "already logged in"),
            PwdTooShort => write!(f, "password must contains more than 8 characters"),
            InvalidUser => write!(f, "invalid user name"),
            Unauthorized => write!(f, "authorization is required"),
        }
    }
}

impl From<sccore::Error> for Error {
    fn from(value: sccore::Error) -> Self 
        {Error::Core(value)}
}