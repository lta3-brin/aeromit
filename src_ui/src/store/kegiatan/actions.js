import axios from "axios"
import {Cookies} from "quasar"

export async function kegiatanAction() {
  let header = Cookies.get("_msk")

  return axios.get(`${process.env.APP_ADDRESS}/v1/kegiatan`, {
    headers: {
      authorization: `Bearer ${header}`
    }
  })
}
