pub const STORE_INDEX: &str = r#"
import { usePageStore } from "./pageStore";

type PageStoreType = ReturnType<typeof usePageStore>;

interface AppStore {
  page: PageStoreType
}
const appStore = {} as AppStore;
export const registerStore = () => {
  appStore.page = usePageStore()
}

export default appStore;
"#;