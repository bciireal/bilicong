<script setup>
import { ref, computed, reactive, onMounted } from "vue";
import { useRouter } from "vue-router";

import { warnDialog, getDevices } from "../services/api";

import MsgFooter from "../components/MsgFooter.vue";
import IconWithMsg from "../components/IconWithMsg.vue";

import connect_lost_icon from "../assets/connect-lost.svg";
import scaning_icon from "../assets/scaning.svg";

const router = useRouter();

const task_locked = ref(false);

const selected = ref("");
const options = reactive([]);

const footer_msg = ref("");

const allow_forward = computed(() => {
  return !task_locked.value && selected.value !== "";
});

const refreshDevice = async () => {
  if (task_locked.value) {
    warnDialog("请不要重复点击");
    return;
  }

  task_locked.value = true;
  options.splice(0);
  selected.value = "";
  footer_msg.value = "";

  let device_list = [];
  try {
    device_list = await getDevices();
  } catch (err) {
    warnDialog(`获取设备列表时出错: ${err}`);
    footer_msg.value = `获取设备列表时出错: ${err}`;
  }

  device_list.forEach((d) => {
    options.push(d);
  });

  task_locked.value = false;
};

const isDeviceAvaliable = (device_info) => {
  return !["offline", "unauthorized"].includes(device_info.label);
};

const pageForward = () => {
  if (!allow_forward.value) {
    warnDialog("必须先选择一个设备");
    return;
  }

  sessionStorage.setItem("device_adb_sid", selected.value);
  router.push("/list");
};

onMounted(refreshDevice);
</script>

<template>
  <header
    class="sticky top-0 p-3 flex items-center bg-zinc-100 border-b border-b-zinc-500"
  >
    <div class="flex-1"></div>
    <div class="flex-1 text-center">
      <p class="font-semibold">1/3 选择设备</p>
    </div>
    <div class="flex-1 text-right">
      <button :disabled="!allow_forward" @click="pageForward">
        去选择视频 &rarr;
      </button>
    </div>
  </header>

  <main class="max-h-full pb-25 overflow-auto flex flex-col items-center">
    <IconWithMsg
      v-if="task_locked"
      :icon="scaning_icon"
      msg="扫描设备中"
      :spin-icon="true"
    />

    <label
      v-for="device_info in options"
      :key="device_info.sid"
      class="flex round-box items-center gap-3 w-[80vw] mt-6"
      :class="{
        'border-blue-500 bg-blue-50': selected === device_info.sid,
        'cursor-not-allowed brightness-80': !isDeviceAvaliable(device_info),
      }"
    >
      <input
        v-model="selected"
        type="radio"
        :value="device_info.sid"
        class="size-4"
        :disabled="!isDeviceAvaliable(device_info)"
      />
      <div class="flex flex-col">
        <div>{{ device_info.sid }}</div>
        <div class="text-sm text-gray-500">{{ device_info.label }}</div>
      </div>
    </label>

    <IconWithMsg
      v-if="options.length === 0 && !task_locked"
      :icon="connect_lost_icon"
      msg="暂无设备, 请检查连接后刷新"
    />
    <button class="my-6" :disabled="task_locked" @click="refreshDevice">
      点我刷新
    </button>
  </main>

  <MsgFooter :msg="footer_msg"></MsgFooter>
</template>
