
const routes = [
  {
    path: '/',
    component: () => import('layouts/MainLayout/MainLayout.vue'),
    children: [
      { path: '', component: () => import('pages/Utama/Utama.vue'), name: 'utama' },
      { path: 'profil', component: () => import('pages/Profil.vue'), name: 'profil' },
      { path: 'masuk', component: () => import('pages/Login/Login.vue'), name: 'login' },
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
