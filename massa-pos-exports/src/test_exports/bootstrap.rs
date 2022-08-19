use std::collections::{BTreeMap, HashMap};

use massa_models::Slot;

use crate::{PoSFinalState, Selection};

/// Compare two PoS States
pub fn assert_eq_pos_state(s1: &PoSFinalState, s2: &PoSFinalState) {
    let mut s1_cleared = s1.cycle_history.clone();
    // remove bootstrap safety cycle from s1
    s1_cleared.pop_back();
    assert_eq!(
        s1_cleared, s2.cycle_history,
        "PoS cycle_history mismatching"
    );
    assert_eq!(
        s1.deferred_credits.0, s2.deferred_credits.0,
        "PoS deferred_credits mismatching"
    );
    for (a, b) in s1_cleared.iter().zip(s2.cycle_history.iter()) {
        assert_eq!(a.roll_counts.len(), b.roll_counts.len());
        for item in a.roll_counts.iter() {
            assert_eq!(
                item,
                b.roll_counts.get_key_value(item.0).unwrap(),
                "ASSERT SHOULD NEVER FAIL"
            );
        }
        for (a1, b1) in a.roll_counts.iter().zip(b.roll_counts.iter()) {
            assert_eq!(a1, b1, "roll_counts order differs");
        }
        assert_eq!(a.production_stats.len(), b.production_stats.len());
        for (a2, b2) in a.production_stats.iter().zip(b.production_stats.iter()) {
            assert_eq!(a2, b2, "production_stats order differs");
        }
    }
}

/// Compare two PoS Selections
pub fn assert_eq_pos_selection(
    s1: &BTreeMap<u64, HashMap<Slot, Selection>>,
    s2: &BTreeMap<u64, HashMap<Slot, Selection>>,
) {
    assert_eq!(s1.len(), s2.len(), "PoS selections len do not match");
    for (key, value) in s2 {
        if let Some(s1_value) = s1.get(key) {
            for (slot, b) in value {
                let a = s1_value.get(slot).unwrap();
                assert_eq!(a, b, "Selection mismatching for {:?}", slot);
            }
        } else {
            panic!("missing key in first selection");
        }
    }
    assert_eq!(s1, s2, "PoS selections do not match");
}
