export function mockApi({ data, delay }: any) {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve(data);
    }, delay || 500);
  });
}
