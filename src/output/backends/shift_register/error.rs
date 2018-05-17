use std::error::Error;
use std::fmt;
use sysfs_gpio::Error as SysFsGpioError;


#[derive(Debug)]
pub enum ShiftRegisterError {
    CouldNotSet,
    CouldNotGet,
    CouldNotUnSet,
    SysFsGpioError(SysFsGpioError),
}

impl fmt::Display for ShiftRegisterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShiftRegisterError::CouldNotSet => write!(f, "Konnte ShiftRegister nicht schreiben"),
            ShiftRegisterError::CouldNotGet => write!(f, "Konnte ShiftRegister nicht schreiben"),
            ShiftRegisterError::CouldNotUnSet => write!(f, "Konnte ShiftRegister nicht schreiben"),
            ShiftRegisterError::SysFsGpioError(ref err) => write!(f, "SysFS GPIO Error: {}", err),
        }
    }
}

impl Error for ShiftRegisterError {
    fn description(&self) -> &str {
        match *self {
            ShiftRegisterError::CouldNotSet => "Konnte ShiftRegister nicht schreiben",
            ShiftRegisterError::CouldNotGet => "Konnte ShiftRegister nicht schreiben",
            ShiftRegisterError::CouldNotUnSet => "Konnte ShiftRegister nicht schreiben",
            ShiftRegisterError::SysFsGpioError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ShiftRegisterError::CouldNotSet => None,
            ShiftRegisterError::CouldNotGet => None,
            ShiftRegisterError::CouldNotUnSet => None,
            ShiftRegisterError::SysFsGpioError(ref err) => Some(err),
        }
    }
}

impl From<SysFsGpioError> for ShiftRegisterError {
    fn from(error: SysFsGpioError) -> Self {
        ShiftRegisterError::SysFsGpioError(error)
    }
}
