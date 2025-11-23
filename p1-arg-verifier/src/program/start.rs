// use
use crate::program::utils::{
    ICON_ERROR, ICON_SUCCESS, MESSAGE_ARG_ERROR, MESSAGE_ARG_SUCCESS, MessageOutput,
};

// ============================================================================
// check_args
fn check_args(args: &[String]) -> Result<String, std::io::Error> {
    let arg1 = match args.get(1) {
        Some(param1) => param1.clone(),
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "InvalidInput.",
            ));
        }
    };
    Ok(arg1)
}

// ============================================================================
// retrieve_message
fn retrieve_message(result: Result<String, std::io::Error>) -> String {
    match result {
        Ok(checker) => {
            format!("{ICON_SUCCESS} {MESSAGE_ARG_SUCCESS} {checker}")
        }
        Err(error) => {
            format!("{ICON_ERROR} {MESSAGE_ARG_ERROR} {error}")
        }
    }
}

// ============================================================================
// check_message
fn check_message(result: Result<String, std::io::Error>) -> MessageOutput {
    let message = retrieve_message(result);

    if message.starts_with(ICON_SUCCESS) {
        MessageOutput::Success(message)
    } else {
        MessageOutput::Error(message)
    }
}

// ============================================================================
// start
pub fn start(args: &[String]) -> MessageOutput {
    let result = check_args(args);
    check_message(result)
}

// ============================================================================
// ============================================================================
// ============================================================================
// TEST
// ============================================================================
// ============================================================================
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::{io, vec};

    // ------------------------------------------------------------------------
    // start
    #[test]
    fn test_start_success() {
        let args = vec!["Param0".to_string(), "Param1".to_string()];
        let result = start(&args);
        let mut message_content: Option<String> = None;

        if let MessageOutput::Success(message) = result {
            message_content = Some(message);
        }

        let final_message = message_content.expect("Test failed: Expected Success."); // final_message has to be Some() not None
        assert!(final_message.starts_with(ICON_SUCCESS));

        let expected_arg = args.get(1).expect("ERROR TEST: param 1 is missing."); // expected_arg has to be 'Param1"
        assert!(final_message.contains(expected_arg));
    }

    #[test]
    fn test_start_error() {
        let args = vec!["Param0".to_string()];
        let result = start(&args);
        let mut message_content: Option<String> = None;

        if let MessageOutput::Error(message) = result.clone() {
            message_content = Some(message);
        }

        let final_message = message_content.expect("Test failed: Expected Error.");

        assert!(final_message.starts_with(ICON_ERROR));
        assert!(final_message.contains(MESSAGE_ARG_ERROR));
        assert_ne!(MessageOutput::Success("Dummy arg.".to_string()), result);
    }

    // ------------------------------------------------------------------------
    // check_message
    #[test]
    fn test_check_message_success_simple() {
        let arg_success = "Minimal arg".to_string();
        let result = Ok(arg_success);
        let output = check_message(result);

        assert!(matches!(output, MessageOutput::Success(_)));
    }

    #[test]
    fn test_check_message_error() {
        let arg_error = io::Error::new(io::ErrorKind::NotFound, "File not found.");
        let result = Err(arg_error);
        let output = check_message(result);

        assert!(matches!(output, MessageOutput::Error(_)));
    }

    // ------------------------------------------------------------------------
    // retrieve_message
    #[test]
    fn test_retrieve_message_success() {
        let arg = "https:://www.badprog.com/".to_string();
        let arg_cloned = arg.clone();
        let result = Ok(arg);
        let expected = format!("{ICON_SUCCESS} {MESSAGE_ARG_SUCCESS} {arg_cloned}");
        let message = retrieve_message(result);

        assert_eq!(expected, message);
    }

    #[test]
    fn test_retrieve_message_error() {
        let error_kind = io::ErrorKind::NotFound;
        let error_detail = "Param 1 not found.";
        let arg = io::Error::new(error_kind, error_detail);
        let arg_copied = io::Error::new(error_kind, error_detail);
        let result = Err(arg);
        let expected = format!("{ICON_ERROR} {MESSAGE_ARG_ERROR} {arg_copied}");
        let message = retrieve_message(result);

        assert_eq!(expected, message);
    }

    // ------------------------------------------------------------------------
    // check_args
    #[test]
    fn test_check_args_with_param1() {
        let expected_arg = "https://www.badprog.com".to_string();
        let args = vec!["param0".to_string(), expected_arg.clone()];
        let result = check_args(&args);
        let param1 = result.expect("There is something wrong :(");

        assert_eq!(expected_arg, param1);
    }

    #[test]
    fn test_check_args_without_param1() {
        let args = vec!["param0".to_string()];
        let result = check_args(&args);
        let option = result.ok(); // converts the Result variant (Err(E)) into an Option variant (None)

        assert_eq!(None, option);
    }
}
