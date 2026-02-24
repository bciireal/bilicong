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

import IconWithMsg from "../components/IconWithMsg.vue";
import ProgressBar from "../components/ProgressBar.vue";

import loading_icon from "../assets/loading.svg";
import download_icon from "../assets/download.svg";

const router = useRouter();

const task_locked = ref(false);

const target_path = ref("");

const icon_msg = ref("");

const current_pulled_index = ref(0);

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

const pullMediaWithContext = async (entry_info) => {
  try {
    await pullMedia(device_sid, target_path.value, entry_info);
  } catch (err) {
    throw new Error(`Error pulling ${JSON.stringify(entry_info)}: ${err}`, {
      cause: err,
    });
  }

  icon_msg.value = `导出中, 当前已完成: ${entry_info.title} P${entry_info.page}`;
  current_pulled_index.value += 1;
};

const startSync = async () => {
  if (task_locked.value) {
    warnDialog("请不要重复点击");
    return;
  }

  icon_msg.value = "导出中";
  task_locked.value = true;
  current_pulled_index.value = 0;

  let tasks = [];
  for (let i = 0; i < selected.length; i++) {
    const entry_info = selected[i];
    tasks.push(pullMediaWithContext(entry_info));
  }

  let tasks_status = await Promise.allSettled(tasks);
  let rejected_tasks = tasks_status
    .filter((s) => s.status === "rejected")
    .map((s) => s.reason);

  if (rejected_tasks.length !== 0) {
    await warnDialog(
      `导出成功 ${tasks_status.length - rejected_tasks.length} 条视频, 导出失败 ${rejected_tasks.length} 条视频:\n${rejected_tasks}`,
    );
  } else {
    await infoDialog(`共导出成功 ${selected.length} 条视频`);
  }

  icon_msg.value = "导出完成";
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

  <main
    class="max-h-full pb-20 overflow-auto flex flex-col items-center gap-10"
  >
    <IconWithMsg
      :icon="task_locked ? loading_icon : download_icon"
      :msg="icon_msg"
      :spin-icon="task_locked"
    />

    <ProgressBar :total="selected.length" :value="current_pulled_index" />

    <div
      class="border border-zinc-500 p-5 rounded-md bg-zinc-50 shadow-md flex flex-col gap-6 w-[80vw]"
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
        导出
      </button>
    </div>
  </main>
</template>
