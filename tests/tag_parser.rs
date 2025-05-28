use prismx::zen::utils::parse_tags;

#[test]
fn parses_hashtags_lowercase() {
    let tags = parse_tags("Working on #FOCUS and #triton items #COIN");
    assert_eq!(tags, vec!["#focus", "#triton", "#coin"]);
}
