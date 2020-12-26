import axios from "axios"

export async function kegiatanAction (context, payload) {
  try {
    let res = await axios.get("http://127.0.0.1:8080/v1/kegiatan")

    context.commit("kegiatanMutation", res.data)
  } catch (err) {
    context.commit("kegiatanErrorMutation", err)
  }
}
