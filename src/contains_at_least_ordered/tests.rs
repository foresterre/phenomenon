use crate::contains_at_least_ordered;
use yare::parameterized;

#[derive(Debug, PartialEq)]
enum Message {
    Hello,
    World,
    IgnoreThis,
    IgnoreThisToo,
}

#[test]
fn empty_set() {
    let actual: &[()] = &[];
    let expected = &[];

    contains_at_least_ordered(actual, expected).assert_this()
}

#[test]
fn allow_ignore_this_to_be_skipped() {
    let actual = vec![Message::Hello, Message::IgnoreThis, Message::World];
    let expected = vec![Message::Hello, Message::World];

    contains_at_least_ordered(actual, expected).assert_this()
}

#[parameterized(
    middle_one = { &[Message::Hello, Message::IgnoreThis, Message::World], &[Message::Hello, Message::World] },
    middle_two = { &[Message::Hello, Message::IgnoreThis, Message::IgnoreThis, Message::World], &[Message::Hello, Message::World] },
    middle_two_variation = { &[Message::Hello, Message::IgnoreThisToo, Message::IgnoreThis, Message::World], &[Message::Hello, Message::World] },
    start_one = { &[Message::IgnoreThis, Message::Hello, Message::World], &[Message::Hello, Message::World] },
    start_two = { &[Message::IgnoreThis, Message::IgnoreThis, Message::Hello, Message::World], &[Message::Hello, Message::World] },
    start_two_variation = { &[Message::IgnoreThis, Message::IgnoreThisToo, Message::Hello, Message::World], &[Message::Hello, Message::World] },
    end_one = { &[Message::Hello, Message::World, Message::IgnoreThis], &[Message::Hello, Message::World] },
    end_two = { &[Message::Hello, Message::World, Message::IgnoreThis, Message::IgnoreThis], &[Message::Hello, Message::World] },
    end_two_variation = { &[Message::Hello, Message::World, Message::IgnoreThis, Message::IgnoreThisToo], &[Message::Hello, Message::World] },
)]
fn ignored_element_variations(actual: &[Message], expected: &[Message]) {
    contains_at_least_ordered(actual, expected).assert_this()
}

#[parameterized(
    intersperse = { &[Message::Hello, Message::IgnoreThis, Message::World, Message::IgnoreThis, Message::Hello], &[Message::Hello, Message::World, Message::Hello] },
    shuffled = { &[Message::World, Message::Hello, Message::IgnoreThis, Message::IgnoreThis, Message::Hello, Message::IgnoreThis], &[Message::World, Message::Hello, Message::Hello] },
)]
fn ignored_shuffled_variations(actual: &[Message], expected: &[Message]) {
    contains_at_least_ordered(actual, expected).assert_this()
}

#[parameterized(
    more_hellos_than_required = { &[Message::Hello, Message::Hello, Message::Hello], &[Message::Hello] },
    more_than_required_with_ignored_elements = { &[Message::Hello, Message::IgnoreThis, Message::IgnoreThis, Message::Hello], &[Message::Hello] },
)]
fn ignored_beyond_expected(actual: &[Message], expected: &[Message]) {
    contains_at_least_ordered(actual, expected).assert_this()
}

#[parameterized(
    too_few_once = { &[], &[Message::Hello] },
    too_few_multiple = { &[], &[Message::Hello, Message::Hello, Message::Hello] },
    with_actuals = { &[Message::Hello], &[Message::Hello, Message::Hello, Message::Hello] },
    with_actuals_variation = { &[Message::Hello, Message::World], &[Message::Hello, Message::Hello] },
)]
fn missing_elements(actual: &[Message], expected: &[Message]) {
    contains_at_least_ordered(actual, expected).assert_invalid()
}

#[parameterized(
    incorrect_order = { &[Message::Hello, Message::World], &[Message::World, Message::Hello] },
    no_dont_ignore_this_expect_it = { &[Message::Hello, Message::IgnoreThis, Message::World], &[Message::World, Message::Hello] },
)]
fn incorrect_ordering(actual: &[Message], expected: &[Message]) {
    contains_at_least_ordered(actual, expected).assert_invalid()
}

// Show that Message::IgnoreMe and Message::IgnoreMeToo aren't special
#[parameterized(
    ignore_this = { &[Message::IgnoreThis], &[Message::IgnoreThis] },
    ignore_this_too = { &[Message::IgnoreThisToo], &[Message::IgnoreThisToo] },
)]
fn dont_ignore_me(actual: &[Message], expected: &[Message]) {
    contains_at_least_ordered(actual, expected).assert_this()
}

#[test]
fn explicit_missing_elements() {
    let actual = vec![
        Message::Hello,
        Message::World,
        Message::World,
        Message::IgnoreThis,
        Message::Hello,
    ];
    let expected = vec![
        Message::Hello,
        Message::World,
        Message::Hello,
        Message::World,
        Message::Hello,
    ];

    let test = contains_at_least_ordered(actual, expected);

    let missing = test.elements_not_found().unwrap();
    let expected_missing = &[Message::World, Message::Hello];
    assert_eq!(missing, expected_missing);
}
