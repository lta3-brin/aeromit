<template>
  <q-page class="q-mb-md">
    <div class="row q-pt-lg q-col-gutter-md"
         :class="koleksi.length === 1 ? '' : 'justify-center'"
    >
      <div class="col" v-if="anyError">
        <q-banner rounded inline-actions class="text-white bg-red">
          {{ errorMessage }}
          <template v-slot:avatar>
            <q-icon name="announcement" color="white" size="md" />
          </template>

          <template v-slot:action v-if="statusCode === 401">
            <q-btn flat color="white" label="LOGIN" @click="goToPage('login')" />
          </template>
        </q-banner>
      </div>

      <div class="col" v-if="!anyError && koleksi.length === 0">
        <q-banner rounded inline-actions class="text-white bg-info">
          Oops... Belum ada kegiatan yang ditemukan.
          <template v-slot:avatar>
            <q-icon name="info" color="white" size="md" />
          </template>
        </q-banner>
      </div>

      <div class="col-12 col-md-5"
           v-for="kegiatan in koleksi" :key="kegiatan['_id']"
           v-else
      >
        <ActivityCard :data="kegiatan" />
      </div>
    </div>
  </q-page>
</template>

<script src="./utama.js" />
