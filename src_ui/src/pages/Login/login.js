import axios from "axios"
import {Cookies} from "quasar"
import urlencoded from "form-urlencoded"
import {validate} from "email-validator"

export default {
  name: 'LoginPage',
  data() {
    return {
      email: null,
      password: null,
      pesan: "",
      errorStatus: false
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

        const token = await this.$store.dispatch(
          "otentikasi/otentikasiAction",
          data
        )

        Cookies.set("_msk", token)
        this.$q.loadingBar.stop()
        // this.$router.push({name: "utama"}).then((_) => _)
      } catch (err) {
        this.errorStatus = true
        this.$q.loadingBar.stop()

        if (err.response) {
          let data = err.response.data
          if (data.hasOwnProperty("pesan")) {
            this.pesan = data["pesan"]
          } else {
            this.pesan = data
          }
        } else {
          this.pesan = err.message
        }
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
