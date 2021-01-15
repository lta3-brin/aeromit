import axios from "axios"
import {Cookies} from "quasar"

export async function kegiatanAction(ctx, payload) {
  const header = Cookies.get("_msk")
  let page = 1

  if (payload) {
    page = payload.page ? payload.page : 1
  }

  return axios.get(`${process.env.APP_ADDRESS}/v1/kegiatan`, {
    headers: {
      authorization: `Bearer ${header}`
    },
    params: {
      skip: (page - 1) * process.env.MAX_PAGE
    }
  })
}
