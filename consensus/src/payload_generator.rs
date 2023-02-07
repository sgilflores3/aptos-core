// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::counters::{NUM_SENDERS_IN_BLOCK, TXN_SHUFFLE_SECONDS};
use aptos_types::transaction::SignedTransaction;
use move_core_types::account_address::AccountAddress;
use std::{
    cmp::min,
    collections::{HashMap, VecDeque},
};

/// Interface to generate payload for the proposal
pub trait PayloadGenerator: Send + Sync {
    fn gen_payload(&mut self, txns: Vec<SignedTransaction>) -> Vec<SignedTransaction>;
}

pub struct NoOplPayloadGenerator {}

impl PayloadGenerator for NoOplPayloadGenerator {
    fn gen_payload(&mut self, txns: Vec<SignedTransaction>) -> Vec<SignedTransaction> {
        txns
    }
}

pub struct SenderAwarePayloadGenerator {
    num_transactions_to_look_ahead: usize,
    sliding_window: SenderAwarePayloadState,
}

impl PayloadGenerator for SenderAwarePayloadGenerator {
    fn gen_payload(&mut self, txns: Vec<SignedTransaction>) -> Vec<SignedTransaction> {
        let _timer = TXN_SHUFFLE_SECONDS.start_timer();
        let num_transactions = txns.len();

        let mut candidate_txns = VecDeque::new();
        for txn in txns {
            candidate_txns.push_back(txn)
        }
        for i in 0..num_transactions {
            let remaining_txns = num_transactions - i;
            let max_lookup = min(self.num_transactions_to_look_ahead, remaining_txns);
            for j in 0..max_lookup {
                let candidate = candidate_txns
                    .pop_front()
                    .expect("Expected transaction in the candidate txns");
                if !self.sliding_window.has_conflict_in_window(&candidate) || j == max_lookup - 1 {
                    // Either we find a transaction that has no conflict or we exhaust all the lookup
                    self.sliding_window.add_transaction(candidate);
                    break;
                } else {
                    candidate_txns.push_back(candidate);
                }
            }
        }
        self.sliding_window.finalize()
    }
}

impl SenderAwarePayloadGenerator {
    pub fn new(conflict_window_size: usize, num_transactions_to_look_ahead: usize) -> Self {
        Self {
            num_transactions_to_look_ahead,
            sliding_window: SenderAwarePayloadState::new(conflict_window_size),
        }
    }
}

/// A state full data structure maintained by the payload generator during payload shuffling. On a
/// high level, it maintains a sliding window of the conflicting transactions, which helps the payload
/// generator include a set of transactions which are non-conflicting with each other within a particular
/// window size.
struct SenderAwarePayloadState {
    // Please note that the start index can be negative in case the window size is larger than the
    // end_index.
    start_index: i64,
    // Hashmap of senders to the number of transactions included in the window for the corresponding
    // sender.
    senders_in_window: HashMap<AccountAddress, usize>,
    // Partially ordered transactions, needs to be updated every time add_transactions is called.
    txns: Option<Vec<SignedTransaction>>,
}

impl SenderAwarePayloadState {
    pub fn new(window_size: usize) -> Self {
        Self {
            start_index: -(window_size as i64),
            senders_in_window: HashMap::new(),
            txns: Some(Vec::new()),
        }
    }

    /// Slides the current window. Essentially, it increments the start_index, end_index and
    /// updates the senders_in_window map.
    pub fn add_transaction(&mut self, txn: SignedTransaction) {
        if self.start_index >= 0 {
            // if the start_index is negative, then no sender falls out of the window.
            let sender = self
                .txns
                .as_mut()
                .expect("add transaction called after finalize")
                .get(self.start_index as usize)
                .expect("Transaction expected")
                .sender();
            self.senders_in_window
                .entry(sender)
                .and_modify(|count| *count -= 1);
        }
        let count = self
            .senders_in_window
            .entry(txn.sender())
            .or_insert_with(|| 0);
        *count += 1;
        self.txns
            .as_mut()
            .expect("add transaction called after finalize")
            .push(txn);
        self.start_index += 1;
    }

    pub fn has_conflict_in_window(&self, txn: &SignedTransaction) -> bool {
        self.senders_in_window
            .get(&txn.sender())
            .map_or(false, |count| *count != 0)
    }

    pub fn finalize(&mut self) -> Vec<SignedTransaction> {
        NUM_SENDERS_IN_BLOCK.set(self.senders_in_window.len() as f64);
        self.txns.take().expect("Finalize already called")
    }
}

#[cfg(test)]
mod tests {
    use crate::payload_generator::{PayloadGenerator, SenderAwarePayloadGenerator};
    use aptos_crypto::{ed25519::Ed25519PrivateKey, PrivateKey, SigningKey, Uniform};
    use aptos_types::{
        chain_id::ChainId,
        transaction::{RawTransaction, Script, SignedTransaction, TransactionPayload},
    };
    use move_core_types::account_address::AccountAddress;
    use rand::{rngs::OsRng, Rng};
    use std::collections::HashSet;

    fn create_signed_transaction(num_transactions: usize) -> Vec<SignedTransaction> {
        let private_key = Ed25519PrivateKey::generate_for_testing();
        let public_key = private_key.public_key();
        let sender = AccountAddress::random();

        let mut transactions = Vec::new();

        for i in 0..num_transactions {
            let transaction_payload =
                TransactionPayload::Script(Script::new(vec![], vec![], vec![]));
            let raw_transaction = RawTransaction::new(
                sender,
                i as u64,
                transaction_payload,
                0,
                0,
                0,
                ChainId::new(10),
            );
            let signed_transaction = SignedTransaction::new(
                raw_transaction.clone(),
                public_key.clone(),
                private_key.sign(&raw_transaction).unwrap(),
            );
            transactions.push(signed_transaction)
        }

        transactions
    }

    #[test]
    fn test_single_user_txns() {
        for i in [5, 10, 100] {
            let txns = create_signed_transaction(i);
            let mut payload_generator = SenderAwarePayloadGenerator::new(10, 10);
            let optimized_txns = payload_generator.gen_payload(txns.clone());
            assert_eq!(txns, optimized_txns)
        }
    }

    #[test]
    fn test_perfect_shuffling() {
        let num_senders = 50;
        let mut txns = Vec::new();
        let mut senders = Vec::new();
        for _ in 0..num_senders {
            let mut sender_txns = create_signed_transaction(10);
            senders.push(sender_txns.get(0).unwrap().sender());
            txns.append(&mut sender_txns);
        }

        let mut payload_generator = SenderAwarePayloadGenerator::new(num_senders - 1, 100);
        let optimized_txns = payload_generator.gen_payload(txns.clone());
        assert_eq!(txns.len(), optimized_txns.len());
        let mut sender_index = 0;
        for txn in optimized_txns {
            assert_eq!(&txn.sender(), senders.get(sender_index).unwrap());
            sender_index = (sender_index + 1) % senders.len()
        }
    }

    #[test]
    fn test_random_shuffling() {
        let mut rng = OsRng;
        let max_senders = 50;
        let max_txn_per_sender = 100;
        let num_senders = rng.gen_range(1, max_senders);
        let mut orig_txns = Vec::new();
        let mut senders = Vec::new();
        let mut orig_txn_set = HashSet::new();
        for _ in 0..num_senders {
            let mut sender_txns = create_signed_transaction(rng.gen_range(1, max_txn_per_sender));
            senders.push(sender_txns.get(0).unwrap().sender());
            orig_txns.append(&mut sender_txns);
        }
        for txn in orig_txns.clone() {
            orig_txn_set.insert(txn.into_raw_transaction());
        }

        let mut payload_generator = SenderAwarePayloadGenerator::new(num_senders - 1, 100);
        let optimized_txns = payload_generator.gen_payload(orig_txns.clone());
        let mut optimized_txn_set = HashSet::new();
        assert_eq!(orig_txns.len(), optimized_txns.len());

        for optimized_txn in optimized_txns {
            assert!(orig_txn_set.contains(&optimized_txn.clone().into_raw_transaction()));
            optimized_txn_set.insert(optimized_txn.into_raw_transaction());
        }

        for orig_txn in orig_txns {
            assert!(optimized_txn_set.contains(&orig_txn.into_raw_transaction()));
        }
    }
}
