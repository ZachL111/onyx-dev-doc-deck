use onyx_dev_doc_deck::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 69, slack: 47, drag: 29, confidence: 87 };
    assert_eq!(review_score(case), 185);
    assert_eq!(review_lane(case), "ship");
}
