use lumen::{fixture_set::FixtureSet, QueryBuilder};

const EXAMPLE_SIZE: usize = 10;

macro_rules! assert_query_has_ids {
    ($result:ident $($id:literal)+) => {
        $(
            assert!($result.contains(&$id));
        )+
    };
}

#[test]
fn all() {
    let fixture_set = build_example_fixture_set(EXAMPLE_SIZE);
    let query = QueryBuilder::new().all().build();
    let result = query.evaluate(&fixture_set.ids());

    assert_query_has_ids!(result 1 2 3 4 5 6 7 9 10);
}

#[test]
fn even() {
    let fixture_set = build_example_fixture_set(EXAMPLE_SIZE);
    let query = QueryBuilder::new().even().build();
    let result = query.evaluate(&fixture_set.ids());

    assert_query_has_ids!(result 2 4 6 8 10);
}

#[test]
fn id() {
    let fixture_set = build_example_fixture_set(EXAMPLE_SIZE);
    let query = QueryBuilder::new().id(1).id(2).id(5).build();
    let result = query.evaluate(&fixture_set.ids());

    assert_query_has_ids!(result 1 2 5);
}

fn build_example_fixture_set(amount: usize) -> FixtureSet {
    let mut f = FixtureSet::new();
    for n in 1..=amount {
        f.create_with_id(n);
    }

    f
}
