use onyx_dev_doc_deck::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 78, capacity: 73, latency: 27, risk: 6, weight: 6 };
    assert_eq!(score(signal), 124);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 89, capacity: 82, latency: 19, risk: 23, weight: 10 };
    assert_eq!(score(signal), 85);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 105, capacity: 83, latency: 12, risk: 7, weight: 13 };
    assert_eq!(score(signal), 241);
    assert_eq!(classify(signal), "accept");
}
