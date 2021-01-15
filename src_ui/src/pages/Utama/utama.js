import ActivityCard from "src/components/ActivityCard/ActivityCard.vue"
import {goToPage} from "src/handlers/menu"
import {checkError} from "src/handlers/error"


export default {
  name: 'PageIndex',
  components: {
    ActivityCard
  },
  data() {
    return {
      anyError: false,
      errorMessage: "",
      statusCode: 0,
      max_page: process.env.MAX_PAGE
    }
  },
  computed: {
    koleksi: function() {
      return this.$store.getters['kegiatan/kegiatanHasilGetter']
    }
  },
  async created() {
    await this.fetchKegiatan()
  },
  methods: {
    async fetchKegiatan(page = 1) {
      this.$q.loadingBar.start()
      try {
        this.$store.commit("kegiatan/kegiatanResetMutation")

        const result = await this.$store.dispatch("kegiatan/kegiatanAction", { page })
        const kegiatan = result["data"]

        this.errorMessage = ""
        this.anyError = false
        this.$q.loadingBar.stop()

        this.$store.commit('kegiatan/kegiatanMutation', kegiatan)
      } catch (err) {
        this.anyError = true
        this.$q.loadingBar.stop()
        this.statusCode = checkError(err, this.$store)

        const hasil = this.$store.getters['kegiatan/kegiatanHasilGetter']
        this.errorMessage = `${hasil}.`
      }
    },
    goToPage
  }
}
