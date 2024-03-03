#[derive(Copy, Clone)]
pub enum ResponseCode {
    Success = 100,
    Error = 200,
    UnknownCommand = 201,
    InvalidParameters = 202,
    ServerBusy = 300
}

pub struct Response {
    pub code: ResponseCode,
    pub message: String
}

impl Response {
    pub fn new(code: ResponseCode, message: String) -> Self {
        Response {code, message}
    }

    pub fn success(result: &str) -> Self {
        Response::new(ResponseCode::Success, result.to_string())
    }

    pub fn error(description: &str) -> Self {
        Response::new(ResponseCode::Error, description.to_string())
    }

    pub fn unknown_command() -> Self {
        Response::new(
            ResponseCode::UnknownCommand,
            String::from("The command specified in the request is not recognized.")
        )
    }

    pub fn invalid_parameters() -> Self {
        Response::new(
            ResponseCode::InvalidParameters,
            String::from("Parameters in the request have an incorrect format or value.")
        )
    }

    pub fn server_busy() -> Self {
        Response::new(
            ResponseCode::ServerBusy,
            String::from("The server is temporarily busy and cannot process the request at the moment.")
        )
    }

    pub fn to_string(&self) -> String {
        format!("Code: {:03} Description: {}", self.code as u32, self.message)
    }
}