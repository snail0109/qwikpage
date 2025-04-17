pub const STORE_INDEX: &str = r#"
import { usePageStore } from "./pageStore";
import { useProjectStore } from "./projectStore";

type PageStoreType = ReturnType<typeof usePageStore>;
type ProjectStoreType = ReturnType<typeof useProjectStore>;

interface AppStore {
  page: PageStoreType
  project: ProjectStoreType
}
const appStore = {} as AppStore;
export const registerStore = () => {
  appStore.page = usePageStore()
  appStore.project = useProjectStore()
}

export default appStore;
"#;