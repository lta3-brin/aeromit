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
          >
            <MenuSettings />
          </q-btn>
        </div>
      </q-toolbar>
    </q-header>

    <InfoDialog />

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script>
import {goToHome} from "../handlers/layouts"
import InfoDialog from "components/layouts/InfoDialog"
import MenuSettings from "components/layouts/MenuSettings";

export default {
  name: 'MainLayout',
  data () {
    return {}
  },
  components: {
    InfoDialog,
    MenuSettings
  },
  methods: {
    goToHome,
    clicked: function() {
      this.$store.commit('layouts/openDialogMutation', true)
    }
  }
}
</script>
