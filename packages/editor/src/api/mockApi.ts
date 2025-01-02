export function mockApi({ data, delay }: any) : Promise<any>{
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve(data);
    }, delay || 500);
  });
}
