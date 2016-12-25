// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {
    errors {
        ApiError(d: String) {
            description("API Error")
            display("API Error: '{}'",d)
        }
    }
}
