import { ref } from "vue";

export const canInstall = ref(false);
export const isStandalone = ref(false);

let deferredPrompt = null;

export function registerSW() {
  // Check if already running in standalone mode
  if (
    window.matchMedia("(display-mode: standalone)").matches ||
    window.navigator.standalone === true
  ) {
    isStandalone.value = true;
  }

  // Register Service Worker
  if ("serviceWorker" in navigator) {
    window.addEventListener("load", () => {
      navigator.serviceWorker
        .register("/sw.js", { scope: "/" })
        .then((reg) => {
          console.log(
            "[PWA] ServiceWorker registered successfully with scope:",
            reg.scope,
          );

          // Check for worker updates
          reg.onupdatefound = () => {
            const installingWorker = reg.installing;
            if (installingWorker) {
              installingWorker.onstatechange = () => {
                if (
                  installingWorker.state === "installed" &&
                  navigator.serviceWorker.controller
                ) {
                  console.log(
                    "[PWA] New content is available; please refresh.",
                  );
                }
              };
            }
          };
        })
        .catch((error) => {
          console.warn("[PWA] ServiceWorker registration failed:", error);
        });
    });
  }

  // Capture beforeinstallprompt event
  window.addEventListener("beforeinstallprompt", (e) => {
    // Prevent default mini-infobar from appearing automatically
    e.preventDefault();
    deferredPrompt = e;
    canInstall.value = true;
    console.log("[PWA] beforeinstallprompt captured, ready to prompt user");
  });

  // Track appinstalled event
  window.addEventListener("appinstalled", () => {
    canInstall.value = false;
    isStandalone.value = true;
    deferredPrompt = null;
    console.log("[PWA] App successfully installed to device homescreen!");
  });
}

export async function promptInstall() {
  if (!deferredPrompt) return false;
  try {
    deferredPrompt.prompt();
    const { outcome } = await deferredPrompt.userChoice;
    console.log("[PWA] User response to install prompt:", outcome);
    deferredPrompt = null;
    canInstall.value = false;
    return outcome === "accepted";
  } catch (err) {
    console.error("[PWA] Error showing install prompt:", err);
    return false;
  }
}
