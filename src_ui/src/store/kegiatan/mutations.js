export function kegiatanMutation (state, payload) {
  state.kegiatan = payload
}

export function muatKegiatanMutation(state, payload) {
  payload.forEach(kegiatan => state.kegiatan.push(payload))
}

export function kegiatanResetMutation (state) {
  state.kegiatan.hasil = []
}
