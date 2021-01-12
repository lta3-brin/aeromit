import {Cookies} from "quasar"
import {validate} from "email-validator"
import {checkError} from "src/handlers/error";

export default {
  name: 'LoginPage',
  data() {
    return {
      email: null,
      password: null,
    }
  },
  beforeRouteEnter(to, from, next) {
    const check = to.matched.some(value => value.name === "masuk")

    if (check) {
      const token = Cookies.get("_msk")

      if (token) next({ path: from.path })
      else next()
    } else next()
  },
  computed: {
    pesan: function () {
      return `${this.$store.getters["kegiatan/kegiatanPesanGetter"]}. ${this.$store.getters["kegiatan/kegiatanHasilGetter"]}`
    },
    showBanner: function () {
      return this.$store.getters["kegiatan/kegiatanSuksesGetter"]
    }
  },
  methods: {
    async onSubmit() {
      this.$q.loadingBar.start()

      try {
        const data = {
          email: this.email,
          password: this.password
        }

        const result = await this.$store.dispatch(
          "otentikasi/otentikasiAction",
          data
        )

        const token = result["data"]["hasil"]

        Cookies.set("_msk", token, {
          expires: process.env.APP_EXPIRE,
          sameSite: 'Lax'
        })

        this.$q.loadingBar.stop()
        this.$store.commit("otentikasi/tokenExistMutation", true)
        this.$router.push({name: "utama"}).then((_) => {})
      } catch (err) {
        this.$q.loadingBar.stop()

        checkError(err, this.$store)
      }
    },
    onReset() {
      this.email = null
      this.password = null
    },
    validateEmail(email) {
      return validate(email)
    }
  }
}
