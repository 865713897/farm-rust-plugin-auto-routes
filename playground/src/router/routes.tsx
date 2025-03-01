// @ts-nocheck
// this file is generated by farm-rust-plugin-auto-routes
// do not change anytime!
import React, { Suspense } from 'react';

function withLazyLoad(LazyComponent) {
  const lazyComponentWrapper = (props) => (
    <Suspense fallback={props.loadingComponent}>
      <LazyComponent {...props} />
    </Suspense>
  );
  return lazyComponentWrapper;
}

export const getRoutes = () => {
  const routes = {'index': {'id':'index','path':'/'},'home': {'name':'Home','requireLayout':true,'id':'home','path':'/home'},'home-me': {'id':'home-me','path':'/home/me'},'404': {'id':'404','path':'/404'},'logout': {'id':'logout','path':'/logout'},'login': {'id':'login','path':'/login'}};
  return {
    routes,
    routeComponents: {
      'index': withLazyLoad(React.lazy(() => import('../pages/index.tsx'))),
      'home': withLazyLoad(React.lazy(() => import('../pages/home/index.tsx'))),
      'home-me': withLazyLoad(React.lazy(() => import('../pages/home/me/index.tsx'))),
      '404': withLazyLoad(React.lazy(() => import('../pages/404.tsx'))),
      'logout': withLazyLoad(React.lazy(() => import('../pages/logout/index.tsx'))),
      'login': withLazyLoad(React.lazy(() => import('../pages/login/index.tsx')))
    },
  };
}
