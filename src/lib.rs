pub(crate) mod abis;
pub(crate) mod address;
pub(crate) mod pb;

use crate::address::STAKINGV1_CONTRACT;
use crate::pb::staking;
use substreams::prelude::*;
use substreams::{log, store::StoreAddInt64, Hex};
use substreams::errors::Error;
use substreams_ethereum::{pb::eth::v2::Block, NULL_ADDRESS};
// use ethabi::ethereum_types::H256;
// use ethabi::Address;
// use tiny_keccak::{Hasher, Keccak};

substreams_ethereum::init!();

// Extracts transfers events from the contract
#[substreams::handlers::map]
fn map_stakings(block: Block) -> Result<staking::Transfers, Error> {
    Ok(staking::Transfers {
        transfers: block
            .events::<abis::stakingv1::events::Transfer>(&[&STAKINGV1_CONTRACT])
            .map(|(transfer, log)| {
                log::info!("CNV Transfer/Staking seen");

                staking::Transfer {
                    trx_hash: log.receipt.transaction.hash.clone(),
                    from: transfer.from,
                    to: transfer.to,
                    token_id: transfer.token_id.to_u64(),
                    ordinal: log.block_index() as u64,
                }
            })
            .collect(),
    })
}

#[substreams::handlers::store]
fn store_stakings(transfers: staking::Transfers, s: StoreAddInt64) {
    log::info!("Concave holders state builder");
    for transfer in transfers.transfers {
        if transfer.from != NULL_ADDRESS {
            log::info!("Found a transfer out {}", Hex(&transfer.trx_hash));
            s.add(transfer.ordinal, generate_key(&transfer.from), -1);
        }

        if transfer.to != NULL_ADDRESS {
            log::info!("Found a transfer in {}", Hex(&transfer.trx_hash));
            s.add(transfer.ordinal, generate_key(&transfer.to), 1);
        }
    }
}

fn generate_key(holder: &Vec<u8>) -> String {
    return format!("total:{}:{}", Hex(holder), Hex(STAKINGV1_CONTRACT));
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
