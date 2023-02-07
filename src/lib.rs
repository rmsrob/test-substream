mod abis;
mod pb;
use hex_literal::hex;
use pb::staking;
use substreams::prelude::*;
use substreams::{log, store::StoreAddInt64, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, NULL_ADDRESS};

// Concave Staking Contract
const TRACKED_CONTRACT: [u8; 20] = hex!("93c3A816242E50Ea8871A29BF62cC3df58787FBD");

substreams_ethereum::init!();

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn map_stakings(blk: eth::Block) -> Result<staking::Transfers, substreams::errors::Error> {
    Ok(staking::Transfers {
        transfers: blk
            .events::<abis::stakingv1::events::Transfer>(&[&TRACKED_CONTRACT])
            .map(|(transfer, log)| {
                substreams::log::info!("CNV Transfer/Staking seen");

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

/// Store the total balance of NFT tokens for the specific TRACKED_CONTRACT by holder
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
    return format!("total:{}:{}", Hex(holder), Hex(TRACKED_CONTRACT));
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
