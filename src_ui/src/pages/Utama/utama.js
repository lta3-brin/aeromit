import ActivityCard from "src/components/ActivityCard/ActivityCard.vue"
import {goToPage} from "src/handlers/menu"
import {checkError} from "src/handlers/error"

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
      this.statusCode = checkError(err, this.$store)

      const hasil = this.$store.getters['kegiatan/kegiatanHasilGetter']
      this.errorMessage = `${hasil}.`
    }
  },
  methods: {
    fetchKegiatan,
    goToPage
  }
}
