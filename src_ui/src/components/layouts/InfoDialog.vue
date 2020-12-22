<template>
  <q-dialog v-model="openDialog">
    <q-card>
      <q-card-section class="row items-center">
        <q-avatar icon="info" color="primary" text-color="white" />
        <span class="q-ml-sm">
          Terima kasih telah menggunakan aplikasi Aeromit BBTA3. Apabila ada pertanyaan, silahkan
          menghubungi kemi.
        </span>
      </q-card-section>

      <q-card-actions align="right">
        <q-btn flat label="TUTUP" color="primary" v-close-popup />
        <q-btn flat label="HUBUNGI KAMI" color="primary" v-close-popup />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script>
export default {
  name: 'DialogComponent',
  data () {
    return {}
  },
  props: {
    open: Boolean
  },
  computed: {
    openDialog: {
      get() { return this.open },
      set(val) { return !this.open }
    }
  }
}
</script>
