pub const VIEW_TEMPLATE: &str = r#"
<template>
  <q-render :elements="pageState.page.pageData.elements || []"></q-render>
</template>
<script setup lang="tsx">
import { onMounted } from "vue";
import { storeToRefs } from "pinia";
import appStore from "@/stores";
const { clearPageInfo, savePageInfo } = appStore.page;
const { pageState } = storeToRefs(appStore.page);

{{ pageInfo }}

onMounted(() => {
  const { pageData: data, ...res } = pageInfo;
  const pageData = JSON.parse(data);
  clearPageInfo();
  savePageInfo({
    ...res,
    pageData,
  });
});
</script>
"#;