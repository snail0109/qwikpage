pub const LOOP_INDEX: &str = r#"
import { defineComponent, ref, provide, watch, inject } from "vue";
import { storeToRefs } from "pinia";
import appStore from "@/stores";
import { commonProps } from "@/types";
import { withInstall } from "@/utils/type";
import { handleApi } from "@/utils/handleApi";
import { Flex } from "ant-design-vue";

const Loop = defineComponent({
  name: "QLoop",
  props: commonProps(),
  inheritAttrs: false,
  setup(props, { expose }) {
    const dataItems = ref<any[]>([]);
    const visible = ref(true);
    const parentContextFunc = inject("useLoopItemValueContext", () => ({}));
    const { rowKey, ...restLayout } = props.config.props;
    const { pageState } = storeToRefs(appStore.page);

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

    return () =>
      visible.value && (
        <div
          style={props.config.style}
          data-id={props.id}
          data-type={props.type}
        >
          {props.elements?.length > 0 && (
            <Flex id={props.id} style={props.config.style} {...restLayout}>
              {dataItems.value.map((item, index) => (
                <LoopItemProvider
                  key={item[rowKey || "id"]}
                  item={item}
                  index={index}
                  id={props.id}
                  parentContext={parentContextFunc()}
                >
                  <q-render elements={props.elements} />
                </LoopItemProvider>
              ))}
            </Flex>
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
    const getLoopContext = () => {
      return {
        ...props.parentContext,
        [props.id]: {
          item: props.item,
          index: props.index,
        },
      };
    };
    provide("useLoopItemValueContext", getLoopContext);

    return () => slots.default?.();
  },
});

export default withInstall(Loop);
"#;