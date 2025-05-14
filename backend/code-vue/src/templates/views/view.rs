pub const VIEW_TEMPLATE: &str = r#"
<template>
  {{vue_template}}
</template>
<script setup lang="tsx">
import { ref, onBeforeMount, onMounted, onUnmounted } from "vue";
import { storeToRefs } from "pinia";
import appStore from "@/stores";
import { createEvents } from "@/utils/util";

let eventFunction: { [key: string]: (params?: any) => void } = {};
const { clearPageInfo, savePageInfo } = appStore.page;
const { pageState } = storeToRefs(appStore.page);
{{pageVariables}}

{{ pageInfo }}

onBeforeMount(() => {
  const { pageData: data, ...res } = pageInfo;
  const pageData = JSON.parse(data);
  clearPageInfo();
  savePageInfo({
    ...res,
    pageData,
  });
  if (pageData && pageData.config) {
    eventFunction = createEvents(pageData.config.events || []);
    // 页面初始化
    eventFunction["onLoad"]?.();
  }
})

onMounted(() => {
  // 实例化挂载完成
  eventFunction["onMount"]?.();
});

onUnmounted(() => {
  // 页面卸载
  eventFunction["onDestory"]?.();
});
</script>
"#;