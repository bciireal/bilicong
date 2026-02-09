<script setup>
import { ref, computed, reactive, onMounted } from "vue";
import { useRouter } from "vue-router";

import { warnDialog, getAllPages, probeEntry } from "../services/api";

import MsgFooter from "../components/MsgFooter.vue";
import IconWithMsg from "../components/IconWithMsg.vue";

import connect_lost_icon from "../assets/connect-lost.svg";
import scaning_icon from "../assets/scaning.svg";
import fallback_img from "../assets/fallback.png";

const router = useRouter();

const task_locked = ref(false);

const selected = ref([]);
const options = reactive([]);

const footer_msg = ref("");

const allow_forward = computed(() => {
  return !task_locked.value && selected.value.length !== 0;
});

const device_sid = sessionStorage.getItem("device_adb_sid");

const pageBack = () => {
  router.push("/");
};

const pageForward = () => {
  if (!allow_forward.value) {
    warnDialog("必须先选择至少一个视频");
    return;
  }

  sessionStorage.setItem("entry_info_list", JSON.stringify(selected.value));
  router.push("/pull");
};

const refreshEntryList = async () => {
  if (task_locked.value) {
    warnDialog("请不要重复点击");
    return;
  }

  task_locked.value = true;
  options.splice(0);
  selected.value.splice(0);
  footer_msg.value = "";

  let pages_list = [];
  try {
    footer_msg.value = "获取视频列表中...";
    pages_list = await getAllPages(device_sid);
  } catch (err) {
    warnDialog(`获取视频列表时出错: ${err}`);
    footer_msg.value = `获取视频列表时出错: ${err}`;
  }

  for (let i = 0; i < pages_list.length; i++) {
    const page_path = pages_list[i];
    try {
      footer_msg.value = `解析路径 ${page_path} 中...`;
      options.unshift(await probeEntry(device_sid, page_path));
    } catch (err) {
      await warnDialog(`解析路径 ${page_path} 时出错: ${err}`);
      footer_msg.value = `解析路径 ${page_path} 时出错: ${err}`;
    }
  }

  footer_msg.value = `获取视频列表完成`;
  task_locked.value = false;
};

const coverImageFallback = (e) => {
  // console.warn(e);
  e.target.src = fallback_img;
};

const optionChooseAll = () => {
  selected.value.splice(0);
  options.forEach((e) => {
    selected.value.push(e);
  });
};

const optionChooseNone = () => {
  selected.value.splice(0);
};

onMounted(refreshEntryList);
</script>

<template>
  <header
    class="sticky top-0 p-3 flex items-center bg-zinc-100 border-b border-b-zinc-500"
  >
    <div class="flex-1">
      <button :disabled="task_locked" @click="pageBack">&larr; 返回</button>
    </div>
    <div class="flex-1 text-center">
      <p class="font-semibold">2/3 选择视频</p>
    </div>
    <div class="flex-1 text-right">
      <button :disabled="!allow_forward" @click="pageForward">
        去选择保存位置 &rarr;
      </button>
    </div>
  </header>

  <div class="fixed z-325 bottom-[15vh] right-[8vw] flex gap-3">
    <button :disabled="task_locked" @click="refreshEntryList">刷新</button>
    <button :disabled="task_locked" @click="optionChooseAll">全选</button>
    <button :disabled="task_locked" @click="optionChooseNone">全不选</button>
  </div>

  <main class="max-h-full pb-25 overflow-auto flex flex-col">
    <IconWithMsg
      v-if="task_locked && options.length === 0"
      :icon="scaning_icon"
      msg="扫描视频中"
      :spin-icon="true"
    />

    <p
      v-if="options.length !== 0"
      class="sticky top-0 text-center border-b border-b-zinc-300 p-1 bg-white z-325"
    >
      共 {{ options.length }} 个视频, 已选 {{ selected.length }} 个
    </p>

    <label
      v-for="entry_info in options"
      :key="entry_info.video_id"
      class="flex items-center gap-5 p-3 border-t border-t-zinc-300"
      :class="{
        'bg-blue-50': selected.includes(entry_info),
      }"
    >
      <input
        v-model="selected"
        type="checkbox"
        class="size-5"
        :value="entry_info"
      />
      <img
        :src="entry_info.cover_url"
        alt="Media Cover"
        class="h-24 w-38 rounded-xl shadow-md"
        @error="coverImageFallback"
      />
      <div class="flex flex-col">
        <div class="mb-1">{{ entry_info.title }}</div>
        <div class="text-xs opacity-70">
          P{{ entry_info.page }} {{ entry_info.page_name }}
        </div>
        <div class="text-sm opacity-70 font-mono">
          {{ entry_info.video_id }}
        </div>
        <div class="text-xs opacity-70">UP: {{ entry_info.uploader }}</div>
      </div>
    </label>

    <IconWithMsg
      v-if="options.length === 0 && !task_locked"
      :icon="connect_lost_icon"
      msg="暂无视频"
    />
  </main>

  <MsgFooter :msg="footer_msg" />
</template>
