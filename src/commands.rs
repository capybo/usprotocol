use crate::response::Response;

pub fn compare_words(mut parameters: Vec<&str>) -> Response {
    if parameters.len() != 2 {
        Response::invalid_parameters();
    }

    let word1 = parameters[0];
    let word2 = parameters[1];

    Response::success(if word1 == word2 { "True" } else { "False" })
}

pub fn minus(parameters: Vec<&str>) -> Response {
    if parameters.len() != 2 {
        Response::invalid_parameters();
    }

    let num1: Result<i32, _> = parameters[0].parse();
    let num2: Result<i32, _> = parameters[1].parse();

    if let (Ok(num1), Ok(num2)) = (num1, num2) {
        let result = num1 - num2;
        Response::success(&result.to_string())
    } else {
        Response::invalid_parameters()
    }
}

pub fn plus(parameters: Vec<&str>) -> Response {
    if parameters.len() != 2 {
        Response::invalid_parameters();
    }

    let num1: Result<i32, _> = parameters[0].parse();
    let num2: Result<i32, _> = parameters[1].parse();

    if let (Ok(num1), Ok(num2)) = (num1, num2) {
        let result = num1 + num2;
        Response::success(&result.to_string())
    } else {
        Response::invalid_parameters()
    }
}
