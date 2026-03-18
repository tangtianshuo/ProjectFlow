<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import Modal from "../../ui/Modal.vue";
import Input from "../../ui/Input.vue";
import Button from "../../ui/Button.vue";
import { useLlmStore } from "../../../stores/llmStore";
import { llmApi } from "../../../lib/api";

interface Props {
  open: boolean;
}

defineProps<Props>();

const emit = defineEmits<{
  (e: "close"): void;
}>();

const llmStore = useLlmStore();

const selectedModel = ref("gpt-4o");
const apiKey = ref("");
const loading = ref(false);
const status = ref<"idle" | "configured" | "not_configured">("idle");

const models = [
  { id: "gpt-4o", name: "GPT-4o" },
  { id: "gpt-4o-mini", name: "GPT-4o Mini" },
  { id: "gpt-4-turbo", name: "GPT-4 Turbo" },
  { id: "gpt-3.5-turbo", name: "GPT-3.5 Turbo" },
];

onMounted(async () => {
  selectedModel.value = llmStore.selectedModel;
  await checkStatus();
});

watch(selectedModel, async () => {
  await checkStatus();
});

async function checkStatus() {
  const hasKey = await llmApi.getKeyStatus(selectedModel.value);
  status.value = hasKey ? "configured" : "not_configured";
}

async function saveKey() {
  if (!apiKey.value.trim()) return;

  loading.value = true;
  try {
    await llmStore.saveApiKey(selectedModel.value, apiKey.value);
    apiKey.value = "";
    status.value = "configured";
  } catch (e) {
    console.error("Failed to save API key:", e);
  } finally {
    loading.value = false;
  }
}

async function deleteKey() {
  loading.value = true;
  try {
    await llmStore.deleteApiKey(selectedModel.value);
    status.value = "not_configured";
  } catch (e) {
    console.error("Failed to delete API key:", e);
  } finally {
    loading.value = false;
  }
}

function close() {
  emit("close");
}
</script>

<template>
  <Modal :open="open" title="API 设置" size="sm" @close="close">
    <div class="space-y-4">
      <!-- Model selector -->
      <div>
        <label class="block text-sm font-medium text-[var(--text-secondary)] mb-2">
          模型
        </label>
        <select
          v-model="selectedModel"
          class="w-full bg-[var(--bg-tertiary)] border border-[var(--border-default)] rounded-lg px-3 py-2.5 text-sm text-[var(--text-primary)] focus:outline-none focus:border-[var(--accent-primary)]"
        >
          <option v-for="model in models" :key="model.id" :value="model.id">
            {{ model.name }}
          </option>
        </select>
      </div>

      <!-- API Key input -->
      <div>
        <label class="block text-sm font-medium text-[var(--text-secondary)] mb-2">
          API Key
        </label>
        <Input
          v-model="apiKey"
          type="password"
          placeholder="sk-..."
        />
      </div>

      <!-- Status indicator -->
      <div class="flex items-center gap-2">
        <div
          :class="[
            'w-2 h-2 rounded-full',
            status === 'configured' ? 'bg-green-500' : 'bg-red-500'
          ]"
        />
        <span class="text-sm text-[var(--text-secondary)]">
          {{ status === "configured" ? "已配置" : "未配置" }}
        </span>
      </div>
    </div>

    <template #footer>
      <div class="flex justify-between">
        <Button
          v-if="status === 'configured'"
          variant="danger"
          size="sm"
          :loading="loading"
          @click="deleteKey"
        >
          删除 Key
        </Button>
        <div v-else />
        <div class="flex gap-2">
          <Button variant="secondary" size="sm" @click="close">
            取消
          </Button>
          <Button
            variant="primary"
            size="sm"
            :loading="loading"
            :disabled="!apiKey.trim()"
            @click="saveKey"
          >
            保存
          </Button>
        </div>
      </div>
    </template>
  </Modal>
</template>
