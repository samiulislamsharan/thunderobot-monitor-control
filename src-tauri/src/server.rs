use crate::ddcci::DdcciController;
use crate::profiles::{apply_profile_to_monitor, load_profiles, save_profiles, Profile};
use crate::resolve_feature_code;
use crate::settings::{save_settings, validate_hotkey_against_settings, AppSettings};
use log::{error, info};
use rust_embed::RustEmbed;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use tiny_http::{Header, Method, Response, Server, StatusCode};

#[derive(RustEmbed)]
#[folder = "../frontend/dist"]
pub struct FrontendAssets;

pub fn get_lan_ip() -> String {
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(addr) = socket.local_addr() {
                let ip = addr.ip().to_string();
                if ip != "0.0.0.0" {
                    return ip;
                }
            }
        }
    }
    "127.0.0.1".to_string()
}

fn guess_mime(path: &str) -> &'static str {
    if path.ends_with(".html") {
        "text/html; charset=utf-8"
    } else if path.ends_with(".js") || path.ends_with(".mjs") {
        "application/javascript; charset=utf-8"
    } else if path.ends_with(".css") {
        "text/css; charset=utf-8"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".ico") {
        "image/x-icon"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".json") {
        "application/json"
    } else if path.ends_with(".webmanifest") {
        "application/manifest+json"
    } else if path.ends_with(".woff2") {
        "font/woff2"
    } else if path.ends_with(".woff") {
        "font/woff"
    } else {
        "application/octet-stream"
    }
}

fn cors_headers() -> Vec<Header> {
    vec![
        Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap(),
        Header::from_bytes(
            &b"Access-Control-Allow-Methods"[..],
            &b"GET, POST, OPTIONS, DELETE, PUT"[..],
        )
        .unwrap(),
        Header::from_bytes(
            &b"Access-Control-Allow-Headers"[..],
            &b"Content-Type, Authorization"[..],
        )
        .unwrap(),
    ]
}

fn json_response(json: &serde_json::Value, status: u16) -> Response<std::io::Cursor<Vec<u8>>> {
    let body = json.to_string().into_bytes();
    let mut headers = cors_headers();
    headers.push(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());

    Response::new(
        StatusCode(status),
        headers,
        std::io::Cursor::new(body.clone()),
        Some(body.len()),
        None,
    )
}

fn empty_response(status: u16) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::new(
        StatusCode(status),
        cors_headers(),
        std::io::Cursor::new(Vec::new()),
        Some(0),
        None,
    )
}

fn read_json_body(request: &mut tiny_http::Request) -> Result<serde_json::Value, String> {
    let mut body = String::new();
    let _ = request.as_reader().read_to_string(&mut body);
    serde_json::from_str(&body).map_err(|e| e.to_string())
}

pub fn start_server(ddcci: Arc<DdcciController>, settings: Arc<Mutex<AppSettings>>, port: u16) {
    let bind_addr = format!("0.0.0.0:{}", port);
    let server = match Server::http(&bind_addr) {
        Ok(s) => {
            info!("Phone Remote server listening on http://{}", bind_addr);
            s
        }
        Err(e) => {
            error!("Failed to bind Phone Remote server to {}: {}", bind_addr, e);
            return;
        }
    };

    let server_arc = Arc::new(server);

    // Spawn 3 background worker threads
    for _ in 0..3 {
        let server = server_arc.clone();
        let ddcci = ddcci.clone();
        let settings = settings.clone();

        std::thread::spawn(move || {
            for request in server.incoming_requests() {
                let method = request.method().clone();
                let url = request.url().to_string();
                let path = url.split('?').next().unwrap_or(&url).to_string();

                if method == Method::Options {
                    let _ = request.respond(empty_response(200));
                    continue;
                }

                // API Routes
                if path.starts_with("/api/") {
                    handle_api(request, &method, &path, &ddcci, &settings, port);
                    continue;
                }

                // Static Web Assets (Frontend)
                handle_static(request, &path);
            }
        });
    }

    // Main worker loop on the current spawned thread
    for request in server_arc.incoming_requests() {
        let method = request.method().clone();
        let url = request.url().to_string();
        let path = url.split('?').next().unwrap_or(&url).to_string();

        if method == Method::Options {
            let _ = request.respond(empty_response(200));
            continue;
        }

        // API Routes
        if path.starts_with("/api/") {
            handle_api(request, &method, &path, &ddcci, &settings, port);
            continue;
        }

        // Static Web Assets (Frontend)
        handle_static(request, &path);
    }
}

fn handle_api(
    mut request: tiny_http::Request,
    method: &Method,
    path: &str,
    ddcci: &Arc<DdcciController>,
    settings: &Arc<Mutex<AppSettings>>,
    port: u16,
) {
    let lan_ip = get_lan_ip();

    // GET /api/monitor
    if path == "/api/monitor" && *method == Method::Get {
        let resp = serde_json::json!({
            "model": "DQ34C240L",
            "vendor": "ThundeRobot",
            "full_name": "ThundeRobot DQ34C240L 34″ Fast VA 250Hz Curved Gaming Monitor",
            "resolution": "3440x1440",
            "refresh_rate": 250,
            "curved": true,
            "panel_type": "Fast VA",
            "aspect_ratio": "21:9 Ultrawide",
            "lan_ip": lan_ip,
            "port": port,
            "local_url": format!("http://localhost:{}", port),
            "phone_url": format!("http://{}:{}", lan_ip, port),
            "telemetry": {
                "v_freq": 250,
                "h_freq": 473.2,
                "usage_hours": 127
            }
        });
        let _ = request.respond(json_response(&resp, 200));
        return;
    }

    // GET /api/capabilities
    if path == "/api/capabilities" && *method == Method::Get {
        let resp = serde_json::json!({
            "capabilities": "(prot(monitor)type(LCD)model(DQ34C240L)cmds(01 02 03 07 0C E3 F3)vcp(02 04 05 08 10 12 14(05 06 08 0B) 16 18 1A 3E 60(0F 11 12) 68(00 01 02 03 04 05) 6C 6E 70 AC AE C0 D6(01 04 05) DC(00 01 02 03 04 05) E3)mswhql(1)asset_eep(40)mccs_ver(2.2))"
        });
        let _ = request.respond(json_response(&resp, 200));
        return;
    }

    // GET /api/vcp/all
    if path == "/api/vcp/all" && *method == Method::Get {
        let brightness = ddcci.get_vcp(0x10).map(|(c, _)| c).unwrap_or(95);
        let contrast = ddcci.get_vcp(0x12).map(|(c, _)| c).unwrap_or(45);
        let picture_mode = ddcci.get_vcp(0xDC).map(|(c, _)| c).unwrap_or(0);
        let color_preset = ddcci.get_vcp(0x14).map(|(c, _)| c).unwrap_or(11);
        let red_gain = ddcci.get_vcp(0x16).map(|(c, _)| c).unwrap_or(50);
        let green_gain = ddcci.get_vcp(0x18).map(|(c, _)| c).unwrap_or(50);
        let blue_gain = ddcci.get_vcp(0x1A).map(|(c, _)| c).unwrap_or(50);
        let red_black_level = ddcci.get_vcp(0x6C).map(|(c, _)| c).unwrap_or(50);
        let green_black_level = ddcci.get_vcp(0x6E).map(|(c, _)| c).unwrap_or(50);
        let blue_black_level = ddcci.get_vcp(0x70).map(|(c, _)| c).unwrap_or(50);
        let input_source = ddcci.get_vcp(0x60).map(|(c, _)| c).unwrap_or(15);
        let overdrive = ddcci.get_vcp(0x68).map(|(c, _)| c).unwrap_or(0);
        let gaming_sharpness = ddcci.get_vcp(0x3E).map(|(c, _)| c).unwrap_or(50);
        let mprt_aim = ddcci.get_vcp(0xE3).map(|(c, _)| c).unwrap_or(0);

        let resp = serde_json::json!({
            "features": {
                "brightness": { "value": brightness, "hex": "0x10" },
                "contrast": { "value": contrast, "hex": "0x12" },
                "picture_mode": { "value": picture_mode, "hex": "0xDC" },
                "color_preset": { "value": color_preset, "hex": "0x14" },
                "red_gain": { "value": red_gain, "hex": "0x16" },
                "green_gain": { "value": green_gain, "hex": "0x18" },
                "blue_gain": { "value": blue_gain, "hex": "0x1A" },
                "red_black_level": { "value": red_black_level, "hex": "0x6C" },
                "green_black_level": { "value": green_black_level, "hex": "0x6E" },
                "blue_black_level": { "value": blue_black_level, "hex": "0x70" },
                "input_source": { "value": input_source, "hex": "0x60" },
                "overdrive": { "value": overdrive, "hex": "0x68" },
                "gaming_sharpness": { "value": gaming_sharpness, "hex": "0x3E" },
                "mprt_aim": { "value": mprt_aim, "hex": "0xE3" }
            }
        });
        let _ = request.respond(json_response(&resp, 200));
        return;
    }

    // POST /api/vcp/raw/direct
    if path == "/api/vcp/raw/direct" && *method == Method::Post {
        if let Ok(body) = read_json_body(&mut request) {
            let code = body.get("code").and_then(|v| v.as_u64()).unwrap_or(0) as u8;
            let val = body.get("value").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
            let _ = ddcci.set_vcp(code, val);
            let resp = serde_json::json!({ "success": true, "code": code, "value": val });
            let _ = request.respond(json_response(&resp, 200));
            return;
        }
        let _ = request.respond(empty_response(400));
        return;
    }

    // POST /api/vcp/reset
    if path == "/api/vcp/reset" && *method == Method::Post {
        if let Ok(body) = read_json_body(&mut request) {
            let r_type = body
                .get("reset_type")
                .or_else(|| body.get("type"))
                .and_then(|v| v.as_str())
                .unwrap_or("all");
            let code = match r_type {
                "luminance" => 0x05,
                "color" => 0x08,
                _ => 0x04,
            };
            let _ = ddcci.set_vcp(code, 1);
            let resp = serde_json::json!({ "success": true, "reset_type": r_type });
            let _ = request.respond(json_response(&resp, 200));
            return;
        }
        let _ = request.respond(empty_response(400));
        return;
    }

    // POST /api/vcp/:name
    if path.starts_with("/api/vcp/") && *method == Method::Post {
        let name = &path["/api/vcp/".len()..];
        if let Ok(body) = read_json_body(&mut request) {
            let val = body.get("value").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
            if let Ok(code) = resolve_feature_code(Some(name), None) {
                let _ = ddcci.set_vcp(code, val);
                let resp = serde_json::json!({
                    "success": true,
                    "feature": name,
                    "code": code,
                    "value": val
                });
                let _ = request.respond(json_response(&resp, 200));
                return;
            }
        }
        let _ = request.respond(empty_response(400));
        return;
    }

    // GET /api/profiles
    if path == "/api/profiles" && *method == Method::Get {
        let profiles = load_profiles();
        let resp = serde_json::json!({ "profiles": profiles });
        let _ = request.respond(json_response(&resp, 200));
        return;
    }

    // POST /api/profiles/:id/apply
    if path.starts_with("/api/profiles/") && path.ends_with("/apply") && *method == Method::Post {
        let prefix_len = "/api/profiles/".len();
        let id_part = &path[prefix_len..path.len() - "/apply".len()];
        let profiles = load_profiles();
        if let Some(p) = profiles.into_iter().find(|x| x.id == id_part) {
            let _ = apply_profile_to_monitor(ddcci, &p);
        }
        let resp = serde_json::json!({ "success": true, "profile_id": id_part });
        let _ = request.respond(json_response(&resp, 200));
        return;
    }

    // POST /api/profiles
    if path == "/api/profiles" && *method == Method::Post {
        if let Ok(body) = read_json_body(&mut request) {
            if let Ok(p) = serde_json::from_value::<Profile>(body) {
                let mut list = load_profiles();
                list.retain(|x| x.id != p.id);
                list.push(p.clone());
                let _ = save_profiles(&list);
                let resp = serde_json::json!({ "success": true, "profile": p });
                let _ = request.respond(json_response(&resp, 200));
                return;
            }
        }
        let _ = request.respond(empty_response(400));
        return;
    }

    // DELETE /api/profiles/:id
    if path.starts_with("/api/profiles/") && *method == Method::Delete {
        let id = &path["/api/profiles/".len()..];
        let mut list = load_profiles();
        list.retain(|x| x.id != id);
        let _ = save_profiles(&list);
        let resp = serde_json::json!({ "success": true });
        let _ = request.respond(json_response(&resp, 200));
        return;
    }

    // GET /api/settings
    if path == "/api/settings" && *method == Method::Get {
        let set = settings.lock().unwrap();
        let resp = serde_json::to_value(&*set).unwrap_or(serde_json::json!({}));
        let _ = request.respond(json_response(&resp, 200));
        return;
    }

    // POST /api/settings
    if path == "/api/settings" && *method == Method::Post {
        if let Ok(body) = read_json_body(&mut request) {
            if let Ok(new_set) = serde_json::from_value::<AppSettings>(body) {
                let _ = save_settings(&new_set);
                ddcci.set_delay(new_set.i2c_delay_ms);
                if let Ok(mut current) = settings.lock() {
                    *current = new_set.clone();
                }
                let resp = serde_json::json!({ "success": true, "settings": new_set });
                let _ = request.respond(json_response(&resp, 200));
                return;
            }
        }
        let _ = request.respond(empty_response(400));
        return;
    }

    // POST /api/hotkeys/validate
    if path == "/api/hotkeys/validate" && *method == Method::Post {
        if let Ok(body) = read_json_body(&mut request) {
            let combo = body
                .get("key_combination")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let aid = body.get("action_id").and_then(|v| v.as_str());
            let current = settings.lock().unwrap();
            let res = validate_hotkey_against_settings(&*current, combo, aid);
            let _ = request.respond(json_response(&res, 200));
            return;
        }
        let _ = request.respond(empty_response(400));
        return;
    }

    // Fallback 404 for unknown /api route
    let _ = request.respond(empty_response(404));
}

fn handle_static(request: tiny_http::Request, path: &str) {
    let clean_path = path.trim_start_matches('/');
    let target = if clean_path.is_empty() {
        "index.html"
    } else {
        clean_path
    };

    let (data, mime) = if let Some(file) = FrontendAssets::get(target) {
        (file.data, guess_mime(target))
    } else if let Some(file) = FrontendAssets::get("index.html") {
        // SPA Fallback for client routing
        (file.data, "text/html; charset=utf-8")
    } else {
        let _ = request.respond(empty_response(404));
        return;
    };

    let mut headers = cors_headers();
    headers.push(Header::from_bytes(&b"Content-Type"[..], mime.as_bytes()).unwrap());

    let body = data.to_vec();
    let resp = Response::new(
        StatusCode(200),
        headers,
        std::io::Cursor::new(body.clone()),
        Some(body.len()),
        None,
    );
    let _ = request.respond(resp);
}
