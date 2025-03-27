
/// 路由模板
pub const ROUTE: &str = r#"
  {
    path: '{path}',
    component: () => import('@/pages/{component}'),
    name: '{component}',
  },"#;

/// 菜单模板
pub const MENU: &str = r#"
  {
    path: '{route_path}',
    name: '{route_name}',
    icon: 'AppstoreOutlined',
  },"#;

/// Vue组件模板
pub const COMPONENT: &str = r#"
<template>
  <div class="page-container">
    <div v-if="loading" class="loading-container">
      <a-spin />
    </div>
    <template v-else>
      {components}
    </template>
  </div>
</template>

<script>
export default {
  name: '{compName}',
  data() {
    return {
      pageData: {page_str},
      loading: false
    }
  },
  mounted() {
    // 初始化页面数据
    this.$store.commit('page/setPageInfo', {
      id: '{page_id}',
      pageData: this.pageData
    });
  }
}
</script>

<style scoped>
.page-container {
  width: 100%;
  min-height: 100vh;
  padding: 20px;
}
.loading-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
}
</style>
"#;
