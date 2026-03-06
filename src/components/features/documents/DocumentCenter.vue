<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { useDocumentStore } from "../../../stores/documentStore";
import { useProjectStore } from "../../../stores/projectStore";
import { VueMonacoEditor } from "@guolao/vue-monaco-editor";
import { marked } from "marked";
import Button from "../../ui/Button.vue";
import Modal from "../../ui/Modal.vue";
import Input from "../../ui/Input.vue";
import Select from "../../ui/Select.vue";

const documentStore = useDocumentStore();
const projectStore = useProjectStore();

const showCreateModal = ref(false);
const showEditor = ref(false);
const previewMode = ref<"edit" | "preview">("edit");
const newDocument = ref({
  title: "",
  projectId: "",
  content: "",
});

const editorContent = ref("");
const editorOptions = {
  minimap: { enabled: false },
  lineNumbers: "on" as const,
  wordWrap: "on" as const,
  fontSize: 14,
  automaticLayout: true,
  theme: "vs-dark",
};

const renderedContent = computed(() => {
  if (!editorContent.value) return "";
  return marked(editorContent.value);
});

onMounted(async () => {
  await documentStore.fetchDocuments();
  await projectStore.fetchProjects();
});

const documents = computed(() => documentStore.documents);
const currentDoc = computed(() => documentStore.currentDocument);

async function createDocument() {
  if (!newDocument.value.title.trim()) return;

  try {
    await documentStore.createDocument({
      title: newDocument.value.title,
      projectId: newDocument.value.projectId || undefined,
      content: newDocument.value.content || undefined,
    });
    showCreateModal.value = false;
    newDocument.value = { title: "", projectId: "", content: "" };
  } catch (e) {
    console.error("Failed to create document:", e);
  }
}

async function openDocument(doc: typeof currentDoc.value) {
  if (!doc) return;
  documentStore.setCurrentDocument(doc);
  editorContent.value = doc.content || "";
  showEditor.value = true;
  previewMode.value = "edit";
}

async function saveDocument() {
  if (!currentDoc.value) return;

  try {
    await documentStore.updateDocument(
      currentDoc.value.id,
      currentDoc.value.title,
      editorContent.value
    );
  } catch (e) {
    console.error("Failed to save document:", e);
  }
}

async function deleteDocument(id: string) {
  if (confirm("确定要删除这个文档吗？")) {
    try {
      await documentStore.deleteDocument(id);
    } catch (e) {
      console.error("Failed to delete document:", e);
    }
  }
}

function closeEditor() {
  showEditor.value = false;
  documentStore.setCurrentDocument(null);
}
</script>

<template>
  <div class="flex h-full flex-col lg:flex-row">
    <!-- Document List - Mobile: Collapsible -->
    <div class="w-full lg:w-80 lg:flex-shrink-0 border-b lg:border-b-0 border-white/5 bg-[#0d0d12]/50 flex flex-col">
      <div class="flex items-center justify-between border-b border-white/5 px-4 py-3 lg:py-4">
        <h2 class="font-semibold text-white">文档中心</h2>
        <Button size="sm" @click="showCreateModal = true">
          <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
        </Button>
      </div>

      <div class="flex-1 overflow-y-auto p-2">
        <div v-if="documents.length === 0" class="py-6 lg:py-8 text-center text-sm text-gray-500">
          暂无文档
        </div>
        <div v-else class="space-y-1">
          <div
            v-for="doc in documents"
            :key="doc.id"
            class="group flex items-center justify-between rounded-lg lg:rounded-xl p-2 lg:p-2.5 transition-all duration-200 hover:bg-white/5"
          >
            <button
              class="flex-1 text-left"
              @click="openDocument(doc)"
            >
              <div class="flex items-center gap-2 lg:gap-3">
                <div class="flex h-7 w-7 lg:h-8 lg:w-8 items-center justify-center rounded-lg bg-gradient-to-br from-indigo-500/20 to-violet-500/20 flex-shrink-0">
                  <svg class="h-3.5 w-3.5 lg:h-4 lg:w-4 text-indigo-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                  </svg>
                </div>
                <div class="min-w-0">
                  <span class="block text-sm font-medium text-white truncate">{{ doc.title }}</span>
                  <span v-if="doc.projectId" class="text-xs text-gray-500 hidden sm:block">
                    {{ projectStore.projects.find(p => p.id === doc.projectId)?.name }}
                  </span>
                </div>
              </div>
            </button>
            <button
              class="opacity-0 transition-opacity group-hover:opacity-100 flex-shrink-0 ml-2"
              @click="deleteDocument(doc.id)"
            >
              <svg class="h-4 w-4 text-gray-500 hover:text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Editor Area -->
    <div class="flex-1 bg-[#0a0a0f] flex flex-col min-h-0">
      <div v-if="!showEditor" class="flex flex-1 items-center justify-center">
        <div class="text-center px-4">
          <div class="mx-auto mb-4 flex h-14 w-14 lg:h-16 lg:w-16 items-center justify-center rounded-2xl bg-white/5">
            <svg class="h-7 w-7 lg:h-8 lg:w-8 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
          </div>
          <p class="text-sm lg:text-base text-gray-500">选择一个文档开始编辑</p>
        </div>
      </div>

      <template v-else>
        <!-- Editor Toolbar -->
        <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2 border-b border-white/5 bg-[#0d0d12]/50 px-3 lg:px-4 py-2">
          <div class="flex items-center gap-2 lg:gap-3">
            <button @click="closeEditor" class="rounded-lg p-1.5 text-gray-500 transition-colors hover:bg-white/5 hover:text-white">
              <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
            <input
              v-model="currentDoc!.title"
              class="border-none bg-transparent text-base lg:text-lg font-medium text-white focus:outline-none focus:ring-0 w-32 lg:w-auto truncate max-w-[150px] sm:max-w-none"
              @blur="saveDocument"
            />
          </div>
          <div class="flex items-center gap-2">
            <!-- View Mode Toggle -->
            <div class="flex rounded-lg bg-white/5 p-0.5">
              <button
                @click="previewMode = 'edit'"
                class="rounded-md px-2 lg:px-3 py-1 text-xs transition-colors"
                :class="previewMode === 'edit' ? 'bg-indigo-500 text-white' : 'text-gray-400 hover:text-white'"
              >
                <span class="hidden sm:inline">编辑</span>
                <span class="sm:hidden">编</span>
              </button>
              <button
                @click="previewMode = 'preview'"
                class="rounded-md px-2 lg:px-3 py-1 text-xs transition-colors"
                :class="previewMode === 'preview' ? 'bg-indigo-500 text-white' : 'text-gray-400 hover:text-white'"
              >
                <span class="hidden sm:inline">预览</span>
                <span class="sm:hidden">预</span>
              </button>
            </div>
            <Button size="sm" @click="saveDocument">
              <span class="hidden sm:inline">保存</span>
              <span class="sm:hidden">存</span>
            </Button>
          </div>
        </div>

        <!-- Editor / Preview Content -->
        <div class="flex-1 flex overflow-hidden min-h-0">
          <!-- Editor -->
          <div v-show="previewMode === 'edit'" class="flex-1 overflow-hidden">
            <VueMonacoEditor
              v-model:value="editorContent"
              language="markdown"
              :options="editorOptions"
              @change="saveDocument"
              class="h-full"
            />
          </div>

          <!-- Preview -->
          <div v-show="previewMode === 'preview'" class="flex-1 overflow-y-auto">
            <div class="prose prose-invert max-w-none p-4 lg:p-6">
              <div class="markdown-body text-gray-300" v-html="renderedContent"></div>
            </div>
          </div>
        </div>
      </template>
    </div>

    <!-- Create Document Modal -->
    <Modal :open="showCreateModal" title="创建新文档" @close="showCreateModal = false">
      <form @submit.prevent="createDocument" class="space-y-4">
        <div>
          <label class="mb-1.5 block text-sm font-medium text-gray-300">文档标题</label>
          <Input v-model="newDocument.title" placeholder="请输入文档标题" required />
        </div>
        <div>
          <label class="mb-1.5 block text-sm font-medium text-gray-300">所属项目（可选）</label>
          <Select v-model="newDocument.projectId">
            <option value="">无（全局文档）</option>
            <option v-for="project in projectStore.projects" :key="project.id" :value="project.id">
              {{ project.name }}
            </option>
          </Select>
        </div>
        <div class="flex justify-end gap-2 pt-4">
          <Button variant="secondary" type="button" @click="showCreateModal = false">取消</Button>
          <Button type="submit">创建</Button>
        </div>
      </form>
    </Modal>
  </div>
</template>

<style>
.markdown-body {
  font-size: 14px;
  line-height: 1.7;
}

.markdown-body h1,
.markdown-body h2,
.markdown-body h3,
.markdown-body h4,
.markdown-body h5,
.markdown-body h6 {
  color: #f8fafc;
  font-weight: 600;
  margin-top: 1.5em;
  margin-bottom: 0.5em;
}

.markdown-body h1 { font-size: 1.75em; }
.markdown-body h2 { font-size: 1.5em; }
.markdown-body h3 { font-size: 1.25em; }
.markdown-body h4 { font-size: 1.1em; }

.markdown-body p {
  margin-bottom: 1em;
}

.markdown-body a {
  color: #818cf8;
  text-decoration: none;
}

.markdown-body a:hover {
  text-decoration: underline;
}

.markdown-body code {
  background: rgba(99, 102, 241, 0.1);
  color: #a5b4fc;
  padding: 0.2em 0.4em;
  border-radius: 4px;
  font-size: 0.9em;
  font-family: 'Fira Code', monospace;
}

.markdown-body pre {
  background: #1a1a25;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  padding: 1em;
  overflow-x: auto;
  margin: 1em 0;
}

.markdown-body pre code {
  background: transparent;
  padding: 0;
  color: #e2e8f0;
}

.markdown-body blockquote {
  border-left: 3px solid #6366f1;
  padding-left: 1em;
  margin: 1em 0;
  color: #94a3b8;
}

.markdown-body ul,
.markdown-body ol {
  padding-left: 1.5em;
  margin-bottom: 1em;
}

.markdown-body li {
  margin-bottom: 0.25em;
}

.markdown-body hr {
  border: none;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  margin: 2em 0;
}

.markdown-body table {
  width: 100%;
  border-collapse: collapse;
  margin: 1em 0;
}

.markdown-body th,
.markdown-body td {
  border: 1px solid rgba(255, 255, 255, 0.1);
  padding: 0.5em 1em;
  text-align: left;
}

.markdown-body th {
  background: rgba(99, 102, 241, 0.1);
  font-weight: 600;
}

.markdown-body img {
  max-width: 100%;
  border-radius: 8px;
  margin: 1em 0;
}
</style>
