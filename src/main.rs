#![feature(path, io, fs)]

extern crate git2;
extern crate "rustc-serialize" as rustc_serialize;

use std::io::Write;
use std::fs::File;
use std::env;
use git2::{Repository, Oid};
use git2::Error as GitError;
use rustc_serialize::json;

#[derive(Clone, RustcDecodable, RustcEncodable)]
struct Commit {
    hash: String,
    author: String,
    date: i64,
    message: String
}

fn fetch_commits(repo: &Repository, start: &Option<Oid>, query: &str, amount: usize) -> Result<Vec<Commit>, GitError> {
    let mut revs = try!(repo.revwalk());
    match *start {
        Some(commit_id) => try!(revs.push(commit_id)),
        _ => try!(revs.push_head()),
    }

    revs.set_sorting(git2::SORT_TOPOLOGICAL | git2::SORT_TIME);

    let commits = revs
    .filter_map(|commit_id| { repo.find_commit(commit_id).ok() })
    .filter_map(|commit| {
        match commit.message().and_then(|msg| { Some(msg.contains(query)) }) {
            Some(true) => Some(commit),
            _ => None
        }
    })
    .map(|commit| {
        let c = Commit {
            hash: commit.id().to_string(),
            author: commit.author().name()
                .or(commit.author().email())
                .unwrap_or("Some dude").trim().to_string(),
            date: commit.time().seconds(),
            message: commit.message().unwrap_or("").trim().to_string()
        };
        c
    })
    .take(amount)
    .collect();

    Result::Ok(commits)
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let start = if args.len() >= 2 { Some(Oid::from_str(&args[1][..]).unwrap()) } else { None };

    let cwd = env::current_dir().unwrap();
    let repo = Repository::open(&cwd.join("rust")).unwrap();

    let mut output = File::create(&cwd.join("log.json")).unwrap();

    let commits = fetch_commits(&repo, &start, "[breaking-change]", 100).unwrap();

    match write!(&mut output, "{}", json::as_pretty_json(&commits)) {
        Ok(_) => println!("wrote commits to `log.json`."),
        Err(e) => panic!("failed to write commits to file: {}", e),
    };
}
