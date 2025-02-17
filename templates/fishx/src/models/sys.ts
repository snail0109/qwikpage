const getOpenKeys = () => {
  const key = `/${window.location.pathname.split('/')[1]}`;
  return [key] || [];
};

interface State {
  error: string;
  openKeys: string[];
}

interface Payload {
  payload: any;
}

export default {
  namespace: 'sys',
  state: {
    error: '',
    openKeys: getOpenKeys(),
  },
  reducers: {
    save(state: State, { payload }: Payload) {
      return { ...state, ...payload };
    },
  },
};
