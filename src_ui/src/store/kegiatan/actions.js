import axios from "axios"
import {Cookies} from "quasar"

export async function kegiatanAction() {
  let header = Cookies.get("_msk")

  return axios.get("http://127.0.0.1:8080/v1/kegiatan", {
    headers: {
      authorization: `Bearer ${header}`
    }
  })
}
