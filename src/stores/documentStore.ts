import { defineStore } from "pinia";
import { ref } from "vue";
import { documentApi, type Document, type CreateDocumentRequest } from "../lib/api";

export const useDocumentStore = defineStore("documents", () => {
  const documents = ref<Document[]>([]);
  const currentDocument = ref<Document | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchDocuments(projectId?: string) {
    loading.value = true;
    error.value = null;
    try {
      if (projectId) {
        documents.value = await documentApi.getByProject(projectId);
      } else {
        documents.value = await documentApi.getAll();
      }
    } catch (e) {
      error.value = String(e);
    } finally {
      loading.value = false;
    }
  }

  async function createDocument(data: CreateDocumentRequest) {
    loading.value = true;
    error.value = null;
    try {
      const doc = await documentApi.create(data);
      documents.value.push(doc);
      return doc;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function updateDocument(id: string, title?: string, content?: string) {
    loading.value = true;
    error.value = null;
    try {
      await documentApi.update(id, title, content);
      const index = documents.value.findIndex((d) => d.id === id);
      if (index !== -1) {
        documents.value[index] = { ...documents.value[index], title: title ?? documents.value[index].title, content: content ?? documents.value[index].content };
      }
      if (currentDocument.value?.id === id) {
        currentDocument.value = { ...currentDocument.value, title: title ?? currentDocument.value.title, content: content ?? currentDocument.value.content };
      }
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function deleteDocument(id: string) {
    loading.value = true;
    error.value = null;
    try {
      await documentApi.delete(id);
      documents.value = documents.value.filter((d) => d.id !== id);
      if (currentDocument.value?.id === id) {
        currentDocument.value = null;
      }
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  function setCurrentDocument(doc: Document | null) {
    currentDocument.value = doc;
  }

  return {
    documents,
    currentDocument,
    loading,
    error,
    fetchDocuments,
    createDocument,
    updateDocument,
    deleteDocument,
    setCurrentDocument,
  };
});
