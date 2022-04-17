use legion::serialize::Canon;
use legion::*;
use masmorra::*;

#[test]
fn serialize_simple_zone() {
    let mut world = World::default();

    let initial_room = make_simple_zone(&mut world);

    world.push((Player, AtRoom { room: initial_room }));

    let entity_serializer = Canon::default();
    let mut registry = Registry::<String>::default();
    registry.register::<Player>("player".to_string());
    registry.register::<AtRoom>("at_room".to_string());
    registry.register::<Description>("description".to_string());
    registry.register::<Exits>("exits".to_string());
    registry.register::<Exit>("exit".to_string());

    let x = world.as_serializable(legion::any(), &registry, &entity_serializer);

    let ejson = serde_json::to_value(&x);
    assert!(
        ejson.is_ok(),
        "World should have serialized; error: {:?}",
        ejson
    );

    let val = ejson.unwrap();
    assert!(val.is_object());

    let map = val.as_object().unwrap();
    // 3 rooms + 1 player
    assert_eq!(map.len(), 4);
}
