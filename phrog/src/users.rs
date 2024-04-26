use std::collections::HashMap;
use std::ops::RangeInclusive;
use uzers::os::unix::UserExt;

const DEFAULT_MIN_UID: uzers::uid_t = 1000;
const DEFAULT_MAX_UID: uzers::uid_t = 60000;

pub fn users() -> HashMap<String, String> {
    let uid_range = uid_range();
    unsafe { uzers::all_users() }
        .filter(|u| uid_range.contains(&u.uid()))
        .map(|u| {
            (
                u.name().to_string_lossy().to_string(),
                u.gecos().to_string_lossy().to_string(),
            )
        })
        .collect()
}

fn uid_range() -> RangeInclusive<uzers::uid_t> {
    let mut min_uid = DEFAULT_MIN_UID;
    let mut max_uid = DEFAULT_MAX_UID;

    if let Ok(defs) = std::fs::read_to_string("/etc/login.defs") {
        for line in defs.lines() {
            if line.starts_with("UID_MIN") {
                min_uid = line
                    .strip_prefix("UID_MIN")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or(DEFAULT_MIN_UID);
            }
            if line.starts_with("UID_MAX") {
                max_uid = line
                    .strip_prefix("UID_MAX")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or(DEFAULT_MIN_UID);
            }
        }
    }

    RangeInclusive::new(min_uid, max_uid)
}
