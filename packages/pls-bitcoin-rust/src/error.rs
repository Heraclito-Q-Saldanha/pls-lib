#[derive(Debug)]
pub enum Error {
	TaprootBuilderError(bitcoin::taproot::TaprootBuilderError),
	TaprootBuilderFinalizeError,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<bitcoin::taproot::TaprootBuilderError> for Error {
	#[inline]
	fn from(value: bitcoin::taproot::TaprootBuilderError) -> Self {
		Self::TaprootBuilderError(value)
	}
}
