
const routes = [
  {
    path: '/',
    component: () => import('layouts/MainLayout/MainLayout.vue'),
    children: [
      { path: '', component: () => import('pages/Utama/Utama.vue'), name: 'utama', meta: { kunci: true } },
      { path: 'profil', component: () => import('pages/Profil/Profil.vue'), name: 'profil', meta: { kunci: true } },
      { path: 'masuk', component: () => import('pages/Login/Login.vue'), name: 'masuk' },
    ]
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '*',
    component: () => import('pages/Error404.vue')
  }
]

export default routes
