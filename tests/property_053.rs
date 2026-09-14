use forgequeue::ids;

#[test]
fn id_053_is_random_uuid_v4() {
    let id = ids::job_id();
    assert_eq!(id.get_version_num(), 4);
}

