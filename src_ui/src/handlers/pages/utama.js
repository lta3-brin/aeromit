import ActivityCard from "components/pages/ActivityCard"
import {goToPage} from "../components/menu"

function fetchKegiatan(payload) {
  return payload.data
}

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
