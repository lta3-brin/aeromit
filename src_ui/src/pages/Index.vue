<template>
  <q-page class="q-mb-md">
    <div class="row q-pt-lg q-col-gutter-md"
         :class="koleksi.length === 1 ? '' : 'justify-center'"
    >
      <div class="col" v-if="koleksi.length === 0">
        <q-banner inline-actions class="text-white bg-info">
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

export default {
  name: 'PageIndex',
  components: {
    ActivityCard
  },
  data() {
    return {
      koleksi: []
    }
  },
  async created() {
    this.$q.loadingBar.start()
    this.koleksi = this.fetchKegiatan(await this.$store.dispatch("kegiatan/kegiatanAction"))
    this.$q.loadingBar.stop()
  },
  methods: {
    fetchKegiatan
  }
}
</script>
