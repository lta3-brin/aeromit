<template>
  <q-layout view="lHh Lpr lFf">
    <q-header>
      <q-toolbar class="bg-dark">
        <q-toolbar-title>
          AEROMIT
        </q-toolbar-title>

        <div class="q-gutter-sm">
          <q-btn
            flat
            dense
            round
            icon="home"
            aria-label="Home"
          />

          <q-btn
            flat
            dense
            round
            icon="info"
            aria-label="info"
          />

          <q-btn
            flat
            dense
            round
            icon="settings"
            aria-label="Menu"
          />
        </div>
      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script>
export default {
  name: 'MainLayout',
  data () {
    return {}
  }
}
</script>
