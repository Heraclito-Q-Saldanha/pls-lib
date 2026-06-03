mod error;

#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm;

use std::str::FromStr;

use bitcoin::Address;
use bitcoin::Network;
use bitcoin::key;
use bitcoin::opcodes;
use bitcoin::script;
use bitcoin::secp256k1;
use bitcoin::taproot;

const NUMS: &str = "50929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0";

pub struct BitcoinMultisig {
	pub multisig_scripts: Vec<script::ScriptBuf>,
	pub spend_info: taproot::TaprootSpendInfo,
	pub address: Address,
}

pub fn create_bitcoin_multisig(
	parts: &[key::UntweakedPublicKey],
	arbitrators: &[key::UntweakedPublicKey],
	quorum: usize,
	network: Network,
) -> error::Result<BitcoinMultisig> {
	let secp = secp256k1::Secp256k1::verification_only();

	let mut multisig_scripts = Vec::new();
	let mut leaves = Vec::new();

	let root_script = build_multisig_leaf(parts);

	multisig_scripts.push(root_script.clone());
	leaves.push((5_u32, root_script));

	let arbitrator_combinations = combine(arbitrators, quorum);

	for participant in parts {
		for combo in &arbitrator_combinations {
			let keys: Vec<_> = std::iter::once(*participant).chain(combo.iter().copied()).collect();
			let script = build_multisig_leaf(&keys);

			multisig_scripts.push(script.clone());
			leaves.push((1_u32, script));
		}
	}

	let builder = taproot::TaprootBuilder::with_huffman_tree(leaves)?;
	let nums = key::XOnlyPublicKey::from_str(NUMS).unwrap();

	let Ok(spend_info) = builder.finalize(&secp, nums) else {
		return Err(error::Error::TaprootBuilderFinalizeError);
	};
	let address = Address::p2tr(&secp, spend_info.internal_key(), spend_info.merkle_root(), network);

	Ok(BitcoinMultisig {
		multisig_scripts,
		spend_info,
		address,
	})
}

fn combine<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
	let n = items.len();

	if k > n {
		return vec![];
	}

	let mut result = Vec::new();
	let mut indices: Vec<usize> = (0..k).collect();

	loop {
		let combination = indices.iter().map(|i| items[*i].clone()).collect();

		result.push(combination);

		let Some(i) = (0..k).rev().find(|i| indices[*i] < n - k + *i) else {
			break;
		};

		indices[i] += 1;

		for j in i + 1..k {
			indices[j] = indices[j - 1] + 1;
		}
	}

	result
}

fn build_multisig_leaf(pubkeys: &[key::UntweakedPublicKey]) -> script::ScriptBuf {
	let mut builder = script::Builder::new();

	for (i, pk) in pubkeys.iter().enumerate() {
		builder = builder.push_x_only_key(pk);

		builder = match i {
			0 => builder.push_opcode(opcodes::all::OP_CHECKSIG),
			_ => builder.push_opcode(opcodes::all::OP_CHECKSIGADD),
		}
	}

	builder
		.push_int(pubkeys.len() as i64)
		.push_opcode(opcodes::all::OP_NUMEQUAL)
		.into_script()
}
