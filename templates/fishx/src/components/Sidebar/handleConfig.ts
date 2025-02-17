function trimSlash(str: string) {
  return str.replace(/^\//, '').replace(/\/$/, '');
}

export default function setMenuKey(items = [], parentKey = ''): any {
  return items.map((item: any) => {
    const { path = '', children = [] } = item;
    const key = [parentKey, trimSlash(path)].join('/');

    if (children.length) {
      return {
        ...item,
        key,
        children: setMenuKey(children, key),
      };
    }

    return { ...item, key };
  });
}
