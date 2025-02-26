import { createContext, useContext } from 'react';

export interface IResourceContextProp {
  resource_type: string;
  onImport: (name: string) => void;
  onDelete: (groupName: string, resourceName: string) => void;
}

interface IResourceProviderProps extends IResourceContextProp {
  children: React.ReactNode;
}

const ResourceContext = createContext<IResourceContextProp | null>(null);

export const ResourceGroupProvider = (props: IResourceProviderProps) => {
  const { children, ...rest } = props;
  return <ResourceContext.Provider value={rest}>{children}</ResourceContext.Provider>;
}

export function useResource<T = any>() {
  return useContext(ResourceContext) as T;
}