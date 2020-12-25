<template>
  <q-card flat bordered>
    <q-card-section>
      <div class="text-overline ellipsis">Tempat pelaksanaan</div>
      <div class="text-h6 q-mt-sm ellipsis-2-lines">Judul Kegiatan Aktif Dilaksanakan</div>
    </q-card-section>

    <q-card-section>
      <div>
        <q-chip icon="bookmark" size="sm">aero</q-chip>
        <q-chip icon="bookmark" size="sm">teknologi</q-chip>
        <q-chip icon="bookmark" size="sm">bbta3</q-chip>
      </div>
    </q-card-section>

    <q-separator />

    <q-card-actions class="flex justify-between">
      <q-btn flat round icon="event" />
      <q-btn flat color="primary">
        Gabung
      </q-btn>
    </q-card-actions>
  </q-card>
</template>

<script>
export default {
  name: 'ActivityPageComponent',
  data () {
    return {}
  }
}
</script>
