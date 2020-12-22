<template>
  <q-dialog v-model="openDialog" persistent transition-show="scale" transition-hide="scale">
    <q-card class="bg-dark text-white">
      <q-card-section>
        <div class="text-h6">✈️ AEROMIT BBTA3</div>
      </q-card-section>

      <q-card-section class="q-pt-none">
        Terima kasih telah menggunakan aplikasi Aeromit BBTA3. Apabila ada pertanyaan, silahkan
        menghubungi kami.🎧
      </q-card-section>

      <q-card-actions align="right" class="bg-white text-dark">
        <q-btn flat label="TUTUP" v-close-popup />
        <q-btn flat label="HUBUNGI KAMI" @click="goToExternal('https://bit.ly/ChatBBTA3')" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script>
import {goToExternal} from "src/handlers/layouts"

export default {
  name: 'DialogComponent',
  data () {
    return {}
  },
  computed: {
    openDialog: {
      get() {
        return this.$store.getters['layouts/openDialogGetter']
      },
      set(val) {
        this.$store.commit('layouts/openDialogMutation', val)
      }
    }
  },
  methods: {
    goToExternal
  }
}
</script>
