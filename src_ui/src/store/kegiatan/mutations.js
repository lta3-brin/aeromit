export function kegiatanMutation (state, payload) {
  state.kegiatan = payload
}

export function muatKegiatanMutation(state, payload) {
  payload.forEach(kegiatan => {
    state.kegiatan.hasil.push(kegiatan)
  })
}

export function kegiatanResetMutation (state) {
  state.kegiatan.hasil = []
}
