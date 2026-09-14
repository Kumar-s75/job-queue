use forgequeue::ids;

#[test]
fn id_018_is_random_uuid_v4() {
    let id = ids::job_id();
    assert_eq!(id.get_version_num(), 4);
}

