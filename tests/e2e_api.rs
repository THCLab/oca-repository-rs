use std::{
    fs,
    net::TcpListener,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::Duration,
};

use reqwest::blocking::Client;
use serde_json::Value;
use tempfile::TempDir;

struct TestServer {
    child: Child,
    base_url: String,
    _tempdir: TempDir,
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn find_free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .and_then(|listener| listener.local_addr())
        .map(|addr| addr.port())
        .expect("failed to find free port")
}

fn write_config(dir: &Path, port: u16) -> PathBuf {
    let config_dir = dir.join("config");
    fs::create_dir_all(&config_dir).expect("failed to create config dir");

    let data_entries_path = dir.join("data_entries");
    let db_path = dir.join("db");
    let cache_path = dir.join("cache");
    let search_path = dir.join("db_search");
    let ocafiles_cache_path = dir.join("oca_repo_cache");

    for p in [
        &data_entries_path,
        &db_path,
        &cache_path,
        &search_path,
        &ocafiles_cache_path,
    ] {
        fs::create_dir_all(p).expect("failed to create data dir");
    }

    let overlay_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("core_overlays");
    let config = format!(
        r#"application:
  host: 127.0.0.1
  port: {port}
  data_entries_path: "{data_entries}"
  log_to_file: false
  log_path: ""
  overlayfile_dir: "{overlay}"

database:
  path: "{db}"

cache_storage:
  path: "{cache}"

search_engine:
  path: "{search}"

ocafiles_cache:
  path: "{ocafiles_cache}"
"#,
        port = port,
        data_entries = data_entries_path.display(),
        overlay = overlay_dir.display(),
        db = db_path.display(),
        cache = cache_path.display(),
        search = search_path.display(),
        ocafiles_cache = ocafiles_cache_path.display(),
    );

    let config_path = config_dir.join("config.yml");
    fs::write(&config_path, config).expect("failed to write config");
    config_path
}

fn wait_for_health(base_url: &str) {
    let client = Client::new();
    let health_url = format!("{}/health_check", base_url);
    for _ in 0..50 {
        if let Ok(resp) = client.get(&health_url).send() {
            if resp.status().is_success() {
                return;
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
    panic!("server did not become ready");
}

fn start_server() -> TestServer {
    let tempdir = TempDir::new().expect("failed to create tempdir");
    let port = find_free_port();
    write_config(tempdir.path(), port);

    let exe = env!("CARGO_BIN_EXE_oca-repository");
    let child = Command::new(exe)
        .current_dir(tempdir.path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to start server");

    let base_url = format!("http://127.0.0.1:{}", port);
    wait_for_health(&base_url);

    TestServer {
        child,
        base_url,
        _tempdir: tempdir,
    }
}

fn load_example_ocafile() -> String {
    let examples_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("ocafile-examples");
    let ocafile_path = examples_root.join("2.0").join("specification").join("examples.ocafile");
    if !ocafile_path.exists() {
        panic!("ocafile example not found: {}", ocafile_path.display());
    }
    fs::read_to_string(ocafile_path).expect("failed to read ocafile example")
}

#[test]
fn e2e_basic_api() {
    let server = start_server();
    let client = Client::new();

    let ocafile = load_example_ocafile();
    let post_url = format!("{}/oca-bundles", server.base_url);
    let resp = client
        .post(&post_url)
        .header("Content-Type", "text/plain")
        .body(ocafile)
        .send()
        .expect("failed to POST ocafile");
    assert!(resp.status().is_success());
    let body: Value = resp.json().expect("invalid json response");
    assert_eq!(body["success"], true);
    let said = body["said"].as_str().expect("missing said").to_string();

    let bundle_url = format!("{}/oca-bundles/{}", server.base_url, said);
    let resp = client.get(&bundle_url).send().expect("GET bundle failed");
    assert!(resp.status().is_success());
    let bundle_json: Value = resp.json().expect("invalid bundle json");
    assert!(bundle_json.get("bundle").is_some());

    let bundle_deps_url = format!("{}?w=true", bundle_url);
    let resp = client
        .get(&bundle_deps_url)
        .send()
        .expect("GET bundle with deps failed");
    assert!(resp.status().is_success());

    let steps_url = format!("{}/steps", bundle_url);
    let resp = client.get(&steps_url).send().expect("GET steps failed");
    assert!(resp.status().is_success());

    let ocafile_url = format!("{}/ocafile", bundle_url);
    let resp = client
        .get(&ocafile_url)
        .send()
        .expect("GET ocafile failed");
    assert!(resp.status().is_success());
    let ocafile_text = resp.text().expect("failed to read ocafile text");
    assert!(ocafile_text.contains("ADD"));

    let objects_url = format!("{}/objects?said={}", server.base_url, said);
    let resp = client
        .get(&objects_url)
        .send()
        .expect("GET objects failed");
    assert!(resp.status().is_success());
    let objects_json: Value = resp.json().expect("invalid objects json");
    assert!(objects_json.get("success").is_some());
    if objects_json["success"] == true {
        assert!(objects_json.get("objects").is_some());
    } else {
        assert!(objects_json.get("errors").is_some());
    }

    let explore_url = format!("{}/explore/{}", server.base_url, said);
    let resp = client
        .get(&explore_url)
        .send()
        .expect("GET explore failed");
    assert!(resp.status().is_success());
    let explore_json: Value = resp.json().expect("invalid explore json");
    assert!(explore_json.get("success").is_some());

    let data_entry_url = format!("{}/data-entry", bundle_url);
    let resp = client
        .get(&data_entry_url)
        .send()
        .expect("GET data-entry failed");
    if cfg!(feature = "data_entries_xls") {
        assert!(resp.status().is_success());
    } else {
        assert_eq!(resp.status().as_u16(), 404);
    }
}
