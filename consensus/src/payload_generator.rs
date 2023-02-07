// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

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
    window_size: usize,
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
            window_size,
            senders_in_window: HashMap::new(),
            txns: Some(Vec::new()),
        }
    }

    /// Slides the current window. Essentially, it increments the start_index, end_index and
    /// updates the senders_in_window map.
    pub fn add_transaction(&mut self, txn: SignedTransaction) {
        if self.start_index >= 0 {
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
        self.txns.take().expect("Finalize already called")
    }
}

#[cfg(test)]
mod tests {}
