use rand::{seq::SliceRandom, thread_rng};
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::time::Duration;

// Change these!
const HYDRA_URL: &str = "https://hydra.nixos.org";

#[derive(Deserialize, Debug, Clone)]
pub struct JobsetEvals {
    pub evals: Box<[Evaluation]>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Evaluation {
    pub builds: Box<[u64]>,
    pub id: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Build {
    //  id: u64,
    buildstatus: u64,
    //  finished: u8,
    nixname: String,
}

fn get_url<T>(url: &str) -> Result<T, reqwest::Error>
where
    T: DeserializeOwned,
{
    Client::new()
        .get(format!("{HYDRA_URL}/{url}"))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .timeout(Duration::new(240, 0))
        .send()?
        .json::<T>()
}

// Should cache this in a file, then make a CLI argument that manually updates it
fn get_latest_evaluation(
    project_name: &str,
    jobset_name: &str,
) -> Result<Evaluation, reqwest::Error> {
    let mut evals =
        get_url::<JobsetEvals>(&format!("/jobset/{project_name}/{jobset_name}/evals"))?.evals;
    evals.sort_by_key(|x| x.id);
    Ok(evals.last().unwrap_or_else(|| panic!(
                "The latest evaluation of jobset {}:{} somehow had no builds. If you see this, let me know.",
                project_name,
                jobset_name)).clone())
}

// See https://github.com/NixOS/hydra/blob/95003f2eb503f71979856470c7caea946f1ae7f0/src/hydra-queue-runner/state.hh#L53
fn is_failing_build(build: &Build) -> bool {
    match build.buildstatus {
        0 => false,   // bsSuccess
        1 => true,    // bsFailed
        2 => true,    // bsDepFailed
        3 => true,    // bsAborted
        4 => false,   // bsCancelled
        6 => true,    // bsFailedWithOutput
        7 => true,    // bsTimedOut
        8 => true,    // bsCachedFailure
        9 => true,    // bsUnsupported
        10 => true,   // bsLogLimitExceeded
        11 => true,   // bsNarSizeLimitExceeded
        12 => true,   // bsNotDeterministic
        100 => false, // bsBusy
        _ => panic!("Unknown build status found"),
    }
}

/// This does not give the error
fn get_failing_builds(builds: &[u64]) -> impl Iterator<Item = Build> + '_ {
    builds
        .iter()
        .flat_map(|x| get_url::<Build>(&format!("/build/{x}")))
        .filter(is_failing_build)
}

fn main() {
    eprintln!("Getting latest nixpkgs:trunk evaluation...");
    let mut builds = get_latest_evaluation("nixpkgs", "trunk").unwrap().builds;
    builds.shuffle(&mut thread_rng());
    eprintln!("Searching for failing builds...");
    let failing_build = get_failing_builds(&builds).next().unwrap();
    println!("{} has failed. Please fix it.", failing_build.nixname);
}
