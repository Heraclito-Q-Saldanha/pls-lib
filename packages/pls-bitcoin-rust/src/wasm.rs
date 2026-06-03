use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Error {
	TaprootBuilderError = "taprootBuilderError",
	TaprootBuilderFinalizeError = "taprootBuilderFinalizeError",
	InvalidNetwork = "invalidNetwork",
	Secp256k1 = "secp256k1",
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[wasm_bindgen]
pub enum Network {
	Bitcoin = "bitcoin",
	Testnet = "testnet",
	Testnet4 = "testnet4",
	Signet = "signet",
	Regtest = "regtest",
}

#[wasm_bindgen]
pub struct BitcoinMultisig {}

impl From<crate::error::Error> for Error {
	#[inline]
	fn from(value: crate::error::Error) -> Self {
		match value {
			crate::error::Error::TaprootBuilderError(_) => Self::TaprootBuilderError,
			crate::error::Error::TaprootBuilderFinalizeError => Self::TaprootBuilderFinalizeError,
		}
	}
}

impl From<bitcoin::secp256k1::Error> for Error {
	#[inline]
	fn from(_: bitcoin::secp256k1::Error) -> Self {
		Self::Secp256k1
	}
}

impl From<crate::BitcoinMultisig> for BitcoinMultisig {
	#[inline]
	fn from(_: crate::BitcoinMultisig) -> Self {
		Self {}
	}
}

impl TryFrom<Network> for bitcoin::Network {
	type Error = Error;

	#[inline]
	fn try_from(value: Network) -> Result<Self, Self::Error> {
		match value {
			Network::Bitcoin => Ok(Self::Bitcoin),
			Network::Testnet => Ok(Self::Testnet),
			Network::Testnet4 => Ok(Self::Testnet4),
			Network::Signet => Ok(Self::Signet),
			Network::Regtest => Ok(Self::Regtest),
			_ => Err(Error::InvalidNetwork),
		}
	}
}

#[wasm_bindgen]
pub fn create_bitcoin_multisig(parts: Vec<String>, arbitrators: Vec<String>, quorum: usize, network: Network) -> Result<BitcoinMultisig, Error> {
	let parts = parts.into_iter().map(|s| s.parse()).collect::<Result<Vec<_>, _>>()?;
	let arbitrators = arbitrators.into_iter().map(|s| s.parse()).collect::<Result<Vec<_>, _>>()?;
	let network = network.try_into()?;

	Ok(crate::create_bitcoin_multisig(&parts, &arbitrators, quorum, network)?.into())
}
