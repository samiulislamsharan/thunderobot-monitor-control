import os
import sys
import subprocess
import socket
import time
import webbrowser
import threading

# Ensure UTF-8 console output on Windows
if sys.stdout and hasattr(sys.stdout, "reconfigure"):
    try:
        sys.stdout.reconfigure(encoding="utf-8")
    except Exception:
        pass


def get_lan_ip() -> str:
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    try:
        s.connect(("10.255.255.255", 1))
        ip = s.getsockname()[0]
    except Exception:
        try:
            ip = socket.gethostbyname(socket.gethostname())
        except Exception:
            ip = "127.0.0.1"
    finally:
        s.close()
    return ip


def check_and_build_frontend():
    root_dir = os.path.dirname(os.path.abspath(__file__))
    dist_dir = os.path.join(root_dir, "frontend", "dist")
    index_file = os.path.join(dist_dir, "index.html")

    if not os.path.exists(index_file):
        print("\n⚙️  First-time launch: building Vue.js 3 frontend...")
        frontend_dir = os.path.join(root_dir, "frontend")

        # Check node_modules
        node_modules = os.path.join(frontend_dir, "node_modules")
        if not os.path.exists(node_modules):
            print("📦 Installing npm dependencies...")
            subprocess.run(
                ["npm.cmd", "install"], cwd=frontend_dir, check=True, shell=True
            )

        print("⚡ Compiling production assets with Vite...")
        subprocess.run(
            ["npm.cmd", "run", "build"], cwd=frontend_dir, check=True, shell=True
        )
        print("✅ Frontend build completed successfully!\n")


def open_browser(url: str):
    time.sleep(1.2)
    try:
        webbrowser.open(url)
    except Exception:
        pass


def main():
    root_dir = os.path.dirname(os.path.abspath(__file__))
    os.chdir(root_dir)
    sys.path.insert(0, root_dir)

    # 1. Build frontend if needed
    check_and_build_frontend()

    lan_ip = get_lan_ip()
    port = 8000
    desktop_url = f"http://localhost:{port}"
    phone_url = f"http://{lan_ip}:{port}"

    print("=" * 72)
    print("  🚀  THUNDEROBOT DQ34C240L DDC/CI MONITOR CONTROL SUITE")
    print("  🖥️   34″ Fast VA 1440p 250Hz Curved Gaming Monitor")
    print("=" * 72)
    print(f"  🖥️  Desktop URL:  {desktop_url}")
    print(f"  📱 Phone / LAN:   {phone_url}")
    print("=" * 72)
    print("  Tip: On your desktop, click 'Phone Remote' to scan the QR code")
    print("  and instantly control your monitor from your smartphone!\n")

    # 2. Open browser in separate thread
    threading.Thread(target=open_browser, args=(desktop_url,), daemon=True).start()

    # 3. Start Uvicorn
    import uvicorn

    uvicorn.run("backend.main:app", host="0.0.0.0", port=port, log_level="info")


if __name__ == "__main__":
    main()
