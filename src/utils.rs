pub fn merge_yaml(a: &mut serde_yaml::Value, b: serde_yaml::Value) {
    match (a, b) {
        (a @ &mut serde_yaml::Value::Mapping(_), serde_yaml::Value::Mapping(b)) => {
            let a = a.as_mapping_mut().unwrap();
            for (k, v) in b {
                if !a.contains_key(&k) {a.insert(k.to_owned(), v.to_owned());}
                else { merge_yaml(&mut a[&k], v); }
            }
        }
        (a, b) => *a = b,
    }
}