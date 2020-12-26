<template>
  <q-card flat bordered>
    <q-card-section>
      <div class="text-overline ellipsis">{{ data['ruang'] }}</div>
      <div class="text-h6 q-mt-sm ellipsis-2-lines">
        {{ data['nama'] }}
      </div>
    </q-card-section>

    <q-card-section>
      <div>
        <q-chip icon="bookmark"
                size="sm"
                v-for="tag in data['tags']"
                :key="tag"
        >
          {{ tag }}
        </q-chip>
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
  props: {
    data: Object
  }
}
</script>
