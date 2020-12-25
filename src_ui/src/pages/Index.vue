<template>
  <q-page class="q-mb-md">
    <div class="row q-pt-lg q-col-gutter-md justify-center">
      <div class="col-12 col-md-5"
           v-for="i in 11" :key="i"
      >
        <ActivityCard />
      </div>
    </div>
  </q-page>
</template>

<script>
import ActivityCard from "components/pages/ActivityCard"

export default {
  name: 'PageIndex',
  components: {
    ActivityCard
  }
}
</script>
