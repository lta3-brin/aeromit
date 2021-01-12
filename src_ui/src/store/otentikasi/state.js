import {Cookies} from "quasar"


export default function () {
  const token = Cookies.get("_msk")

  return {
    tokenExist: !!token
  }
}
