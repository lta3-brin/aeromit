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
            @click="goToHome"
          />

          <q-btn
            flat
            dense
            round
            icon="info"
            aria-label="info"
            @click="clicked"
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

    <Dialog />

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script>
import {goToHome} from '../handlers/layouts'
import Dialog from "components/layouts/InfoDialog";

export default {
  name: 'MainLayout',
  data () {
    return {}
  },
  components: {
    Dialog
  },
  methods: {
    goToHome,
    clicked: function() {
      this.$store.commit('layouts/openDialogMutation', true)
    }
  }
}
</script>
