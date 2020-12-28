<template>
  <q-page class="q-mb-md">
    <div class="row q-pt-lg q-col-gutter-md"
         :class="koleksi.length === 1 ? '' : 'justify-center'"
    >
      <div class="col" v-if="anyError">
        <q-banner rounded inline-actions class="text-white bg-red">
          {{ errorMessage }}
          <template v-slot:avatar>
            <q-icon name="announcement" color="white" size="md" />
          </template>

          <template v-slot:action v-if="statusCode === 401">
            <q-btn flat color="white" label="LOGIN" @click="goToPage('login')" />
          </template>
        </q-banner>
      </div>

      <div class="col" v-if="!anyError && koleksi.length === 0">
        <q-banner rounded inline-actions class="text-white bg-info">
          Oops... Belum ada kegiatan yang ditemukan.
          <template v-slot:avatar>
            <q-icon name="info" color="white" size="md" />
          </template>
        </q-banner>
      </div>

      <div class="col-12 col-md-5"
           v-for="kegiatan in koleksi" :key="kegiatan['_id']"
           v-else
      >
        <ActivityCard :data="kegiatan" />
      </div>
    </div>
  </q-page>
</template>

<script>
import ActivityCard from "components/pages/ActivityCard"
import {fetchKegiatan} from "../handlers/pages/utama"
import {goToPage} from "../handlers/components/menu"

export default {
  name: 'PageIndex',
  components: {
    ActivityCard
  },
  data() {
    return {
      anyError: false,
      errorMessage: "",
      koleksi: [],
      statusCode: 0
    }
  },
  async created() {
    this.$q.loadingBar.start()
    try {
      const kegiatan = this.fetchKegiatan(await this.$store.dispatch("kegiatan/kegiatanAction"))

      this.errorMessage = ""
      this.anyError = false
      this.$q.loadingBar.stop()

      this.$store.commit('kegiatan/kegiatanMutation', kegiatan)
      this.koleksi = this.$store.getters['kegiatan/kegiatanHasilGetter']
    } catch (err) {
      this.anyError = true
      this.$q.loadingBar.stop()

      if (err.response) {
        this.statusCode = err.response.status
        this.$store.commit('kegiatan/kegiatanMutation', err.response.data)
      } else {
        this.statusCode = 500
        this.$store.commit('kegiatan/kegiatanMutation', {
          sukses: false,
          pesan: "Terjadi kesalahan yang perlu diperhatikan",
          hasil: err.message
        })
      }

      const hasil = this.$store.getters['kegiatan/kegiatanHasilGetter']
      this.errorMessage = `${hasil}.`

    }
  },
  methods: {
    fetchKegiatan,
    goToPage
  }
}
</script>
