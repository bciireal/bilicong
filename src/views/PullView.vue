<script setup>
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";

import {
  warnDialog,
  requestDir,
  openDir,
  pullMedia,
  infoDialog,
} from "../services/api";

import MsgFooter from "../components/MsgFooter.vue";
import IconWithMsg from "../components/IconWithMsg.vue";

import download_icon from "../assets/download.svg";

const router = useRouter();

const task_locked = ref(false);

const target_path = ref("");

const footer_msg = ref("");
const icon_msg = ref("");

const device_sid = sessionStorage.getItem("device_adb_sid");
const selected = JSON.parse(sessionStorage.getItem("entry_info_list"));

const pageBack = () => {
  router.push("/list");
};

const changeTargetDir = async () => {
  let r = await requestDir();
  if (r !== null) {
    target_path.value = r;
  }
};

const openTargetDir = () => {
  openDir(target_path.value);
};

const startSync = async () => {
  if (task_locked.value) {
    warnDialog("请不要重复点击");
    return;
  }

  task_locked.value = true;

  for (let i = 0; i < selected.length; i++) {
    const entry_info = selected[i];
    icon_msg.value = `导出中: ${i + 1}/${selected.length} 当前视频 ${entry_info.title}`;

    try {
      await pullMedia(device_sid, target_path.value, entry_info);
    } catch (err) {
      await warnDialog(`导出视频 ${entry_info.title} 时出错: ${err}`);
      footer_msg.value = `导出视频 ${entry_info.title} 时出错: ${err}`;
    }
  }

  icon_msg.value = "导出完成";
  await infoDialog("导出完成");
  task_locked.value = false;
};

onMounted(async () => {
  await changeTargetDir();
  if (target_path.value === "") {
    return;
  }

  await startSync();
  openTargetDir();
});
</script>

<template>
  <header
    class="sticky top-0 p-3 flex items-center bg-zinc-100 border-b border-b-zinc-500"
  >
    <div class="flex-1">
      <button :disabled="task_locked" @click="pageBack">&larr; 返回</button>
    </div>
    <div class="flex-1 text-center">
      <p class="font-semibold">3/3 选择保存位置</p>
    </div>
    <div class="flex-1 text-right"></div>
  </header>

  <main class="max-h-full pb-25 overflow-auto flex flex-col">
    <IconWithMsg :icon="download_icon" :msg="icon_msg" />

    <div
      class="border border-zinc-500 mx-15 mt-15 p-5 rounded-md bg-zinc-50 shadow-md flex flex-col gap-6"
    >
      <div class="flex justify-between">
        <div class="flex flex-col gap-1">
          <p>导出位置</p>
          <p class="text-xs text-zinc-600">
            {{ target_path === "" ? "未设置" : target_path }}
          </p>
        </div>
        <div>
          <button
            class="text-sm mr-3"
            :disabled="task_locked"
            @click="changeTargetDir"
          >
            更改位置
          </button>
          <button
            class="text-sm"
            :disabled="target_path === ''"
            @click="openTargetDir"
          >
            打开文件夹
          </button>
        </div>
      </div>

      <button :disabled="task_locked || target_path === ''" @click="startSync">
        It's MyGO!!!!!
      </button>
    </div>
  </main>

  <MsgFooter :msg="footer_msg" />
</template>
