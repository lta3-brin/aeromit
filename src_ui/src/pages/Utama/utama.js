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
      page: 1,
      showLoadMore: true,
      max_page: process.env.MAX_PAGE
    }
  },
  computed: {
    koleksi: function() {
      return this.$store.getters["kegiatan/kegiatanHasilGetter"]
    }
  },
  async created() {
    await this.fetchKegiatan()
  },
  methods: {
    async fetchKegiatan(page = 1) {
      this.$q.loadingBar.start()
      try {
        const result = await this.$store.dispatch("kegiatan/kegiatanAction", { page })
        const kegiatan = result["data"]

        this.errorMessage = ""
        this.anyError = false
        this.$q.loadingBar.stop()

        if (page === 1) this.$store.commit("kegiatan/kegiatanMutation", kegiatan)
        else this.$store.commit("kegiatan/muatKegiatanMutation", kegiatan["hasil"])

        if (kegiatan["hasil"].length === 0) this.showLoadMore = false
      } catch (err) {
        this.anyError = true
        this.$q.loadingBar.stop()
        this.statusCode = checkError(err, this.$store)

        const hasil = this.$store.getters['kegiatan/kegiatanHasilGetter']
        this.errorMessage = `${hasil}.`
      }
    },
    async muatKegiatanLagi() {
      this.page += 1

      await this.fetchKegiatan(this.page)
    },
    goToPage,
  }
}
