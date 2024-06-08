mod reusable;

#[test]
fn it_s_rect() {
    let rect = test_proj::Rectangle::new(24, 43);
    reusable::this_is_reusable_in_test();
    assert_eq!(rect.width, 24);
}
