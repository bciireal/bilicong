import { invoke } from "@tauri-apps/api/core";
import { open, message } from "@tauri-apps/plugin-dialog";
import { openPath } from "@tauri-apps/plugin-opener";

export async function getDevices() {
  return await invoke("get_devices");
}

export async function getAllPages(deviceId) {
  return await invoke("get_all_pages", { sid: deviceId });
}

export async function probeEntry(deviceId, pagePath) {
  return await invoke("probe_entry", { sid: deviceId, pagePath: pagePath });
}

export async function pullMedia(deviceId, targetPath, entryInfo) {
  return await invoke("pull_media", {
    sid: deviceId,
    targetPath: targetPath,
    entryInfo: entryInfo,
  });
}

export async function requestDir() {
  return await open({ directory: true });
}

export async function openDir(dir) {
  openPath(dir);
}

function addLineBreaks(str) {
  return str.toString().replace(/(.{40})/g, "$1\n");
}

export async function warnDialog(msg) {
  return await message(addLineBreaks(msg), { kind: "warning" });
}

export async function infoDialog(msg) {
  return await message(addLineBreaks(msg), { kind: "info" });
}
