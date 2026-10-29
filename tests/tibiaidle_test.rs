use bot_tibiaidle_rust::game::catalog::{Catalog, ALL_KNOWN_HUNT_IDENTIFIERS};

#[test]
fn test_tibia_hunts_catalog_coverage() {
    assert_eq!(ALL_KNOWN_HUNT_IDENTIFIERS.len(), 113);
    assert!(Catalog::has_route("hunt-26"));
    assert!(Catalog::has_route("hunt-220"));
    assert!(!Catalog::has_route("hunt-999"));
}

#[test]
fn test_tibia_route_solver() {
    let catalog = Catalog::default();
    let area_low = catalog.select_best_hunt(10);
    assert_eq!(area_low.id, 26);

    let area_mid = catalog.select_best_hunt(100);
    assert_eq!(area_mid.id, 46);

    let area_high = catalog.select_best_hunt(1000);
    assert_eq!(area_high.id, 220);
}
