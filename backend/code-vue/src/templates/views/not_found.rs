pub const NOT_FOUND_TEMPLATE: &str = r#"
<template>
  <a-result status="404" title="404" sub-title="抱歉，您当前访问的页面不存在">
    <template #extra>
      <span class="ant-result-subtitle">以下页面可访问:</span>
      <ul>
        <li v-for="route in availableRoutes" :key="route.path">
          <router-link :to="route.path">{{ route.path === '/' ? route.path : '/' + route.name }}</router-link>
        </li>
      </ul>
    </template>
  </a-result>
</template>

<script setup lang="ts">
import { useRouter } from "vue-router";
import { computed } from "vue";

const router = useRouter();
const availableRoutes = computed(() => {
  const list = router.getRoutes().filter((route) => route.name && route.name !== "NotFound");
  const defaultIndex = list.findIndex((route) => route.path === "/");
  if (defaultIndex !== -1) {
    const defaultRoute = list.splice(defaultIndex, 1)[0];
    list.unshift(defaultRoute);
  }
  return list;
});
</script>

<style scoped>
ul {
  list-style: none;
  padding: 0;
}

li {
  margin: 10px 0;
}
</style>
"#;