export function kegiatanMutation (state, payload) {
  state.kegiatan = payload
}

export function kegiatanResetMutation (state) {
  state.kegiatan.hasil = []
}
