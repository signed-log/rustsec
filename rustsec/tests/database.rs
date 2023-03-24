#![cfg(feature = "git")]

use cargo_lock::Lockfile;
use once_cell::sync::Lazy;
use rustsec::{database::Query, repository::git::Repository, Database};
use std::{path::Path, sync::Mutex};

static DEFAULT_DATABASE: Lazy<Mutex<Database>> = Lazy::new(|| {
    Mutex::new(
        Database::fetch()
            .expect("Should be fetchable."),
    )
});

#[test]
fn enumerate_vulnerabilities() {
    let lockfile_path = Path::new("./tests/support/cratesio_cargo.lock");
    let lockfile =
        Lockfile::load(lockfile_path).expect("Should find the lock file in support folder.");
    let db = DEFAULT_DATABASE.lock().unwrap();
    let vuln = db.vulnerabilities(&lockfile);
    assert_eq!(vuln.len(), 1);
}

#[test]
fn query_vulnerabilitie() {
    let lockfile_path = Path::new("./tests/support/cratesio_cargo.lock");
    let lockfile =
        Lockfile::load(lockfile_path).expect("Should find the lock file in support folder.");
    let db = DEFAULT_DATABASE.lock().unwrap();
    let vuln_all = db.query_vulnerabilities(&lockfile, &Query::crate_scope());
    let vuln = db.vulnerabilities(&lockfile);
    assert_eq!(vuln_all, vuln);
}
