use custom_error::custom_error;

custom_error! {pub ArithmeticError
    InvalidInput = "An invalid trit value was encountered. This error is fatal"
}

custom_error! {pub MachineError
    Arithmetic{error: ArithmeticError} = "The machine has encountered a fatal error: {error}",
    DecConversion = "Unable to convert Decimal to Balanced ternary"
}
