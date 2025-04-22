pub const LOOP_INDEX: &str = r#"

import {
  defineComponent,
  ref,
  provide,
  watch,
  onMounted,
  computed,
  inject,
} from "vue";
import { storeToRefs } from "pinia";
import appStore from "@/stores";
import { commonProps } from "@/types";
import { withInstall } from "@/utils/type";
import { handleApi } from "@/utils/handleApi";
import { Row } from "ant-design-vue";

const Loop = defineComponent({
  name: "QLoop",
  props: commonProps(),
  inheritAttrs: false,
  setup(props, { expose }) {
    const dataItems = ref<any[]>([]);
    const visible = ref(true);
    const parentContext = inject("useLoopItemValueContext", {});
    const { rowKey, ...restLayout } = props.config.props;
    const { pageState } = storeToRefs(appStore.page);
    const variableData = computed(
      () => pageState.value.page.pageData.variableData
    );

    const getDataList = async (params: any = {}) => {
      try {
        const res = await handleApi(props.config.api, params);
        if (!Array.isArray(res.data)) {
          dataItems.value = [];
        } else {
          dataItems.value = res.data;
        }
      } catch (error) {
        dataItems.value = [];
      }
    };

    // 计算数据源
    watch(
      () => [props.config.api, pageState.value.page.pageData.variableData],
      () => {
        getDataList({});
      },
      { immediate: true }
    );

    expose({
      show: () => (visible.value = true),
      hide: () => (visible.value = false),
      async search(searchQuery: any) {
        await getDataList(searchQuery);
      },
      async reload() {
        await getDataList({});
      },
      clearData: () => (dataItems.value = []),
    });

    provide("useLoopItemValueContext", {
      ...parentContext,
      [props.id]: computed(() => ({
        item: dataItems.value,
        index: 0,
      })),
    });

    return () =>
      visible.value && (
        <div
          style={props.config.style}
          data-id={props.id}
          data-type={props.type}
        >
          {props.elements?.length > 0 && (
            <Row id={props.id} style={props.config.style} {...restLayout}>
              {dataItems.value.map((item, index) => (
                <LoopItemProvider
                  key={item[rowKey || "id"]}
                  item={item}
                  index={index}
                  id={props.id}
                  parentContext={parentContext}
                >
                  <q-render elements={props.elements} />
                </LoopItemProvider>
              ))}
            </Row>
          )}
        </div>
      );
  },
});

const LoopItemProvider = defineComponent({
  name: "LoopItemProvider",
  props: {
    item: Object,
    index: Number,
    id: String,
    parentContext: Object,
  },
  setup(props, { slots }) {
    provide("useLoopItemValueContext", {
      ...props.parentContext,
      [props.id]: computed(() => ({
        item: props.item,
        index: props.index,
      })),
    });

    return () => slots.default?.();
  },
});

export default withInstall(Loop);
"#;