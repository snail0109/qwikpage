import React from 'react';
import { createBrowserRouter, Navigate } from 'react-router-dom';
import { lazyLoad } from '@/components/LazyLoad';
import ErrorBoundary from '@/components/ErrorBoundary';
import RootLayout from '@/layout/RootLayout';

/**
 * 定义页面路由
 */
export const router = [
  {
    path: '/',
    element: <RootLayout />,
    errorElement: <ErrorBoundary />,
    children: [
      {
        index: true,
        element: <Navigate to="/projects" />,
      },
      {
        path: '/projects',
        element: lazyLoad(React.lazy(() => import('@/pages/home/project/index'))),
      },
      {
        path: '/pages',
        element: lazyLoad(React.lazy(() => import('@/pages/home/project/Pages'))),
      },
      {
        path: '/project/pages',
        element: lazyLoad(React.lazy(() => import('@/pages/home/project/Pages'))),
      },
      {
        path: '/editor/:id',
        element: lazyLoad(React.lazy(() => import('@/layout/EditLayout'))),
        children: [
          {
            path: '/editor/:id/edit',
            element: lazyLoad(
              React.lazy(() => import('@/pages/editor/editor')),
              true,
            ),
          },
          {
            path: '/editor/:id/template',
            element: lazyLoad(
              React.lazy(() => import('@/pages/editor/editor')),
              true,
            ),
          },
        ],
      },
      {
        path: '/project/:id',
        element: lazyLoad(React.lazy(() => import('@/pages/admin/admin'))),
        children: [
          {
            path: '/project/:id/config',
            element: lazyLoad(React.lazy(() => import('@/pages/admin/config/index'))),
          },
          {
            path: '/project/:id/menu',
            element: lazyLoad(React.lazy(() => import('@/pages/admin/menu/index'))),
          },
        ],
      },
      {
        path: '*',
        element: <Navigate to="/404" />,
      },
      {
        path: '/404',
        element: lazyLoad(React.lazy(() => import('@/pages/404'))),
      },
      {
        path: '/403',
        element: lazyLoad(React.lazy(() => import('@/pages/403'))),
      },
    ],
  },
];

export default createBrowserRouter(router);
