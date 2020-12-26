export function kegiatanMutation (state, payload) {
  state.kegiatan = payload
}

export function kegiatanResetMutation (state) {
  state.kegiatan.hasil = []
}

export function kegiatanErrorMutation (state, payload) {
  state.kegiatan_error = payload
}
