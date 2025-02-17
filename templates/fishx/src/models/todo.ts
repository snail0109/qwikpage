import { getAsync } from '@/services';

interface State {
  greeting: string;
  data: any[];
}

interface Payload {
  payload: any;
}

export default {
  namespace: 'todo',
  state: {
    greeting: '',
    data: [],
  },
  reducers: {
    set(state: State, { payload }: Payload) {
      return { ...state, ...payload };
    },

    save(state: State, { payload }: Payload) {
      return { ...state, ...payload };
    },
  },
  effects: {
    *setAsync({ payload: todo }: Payload, { put, call }: any) {
      const data = yield call(getAsync, todo);
      yield put({
        type: 'save',
        payload: {
          data,
        },
      });
    },
  },
};
