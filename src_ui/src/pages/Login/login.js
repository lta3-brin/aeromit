import axios from "axios"
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
        let data = {
          email: this.email,
          password: this.password
        }

        let res = await axios.post(
          "http://localhost:8080/v1/pengguna/login/",
          urlencoded(data),
          { withCredentials: true }
        )

        console.log(res)
        this.$q.loadingBar.stop()
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
