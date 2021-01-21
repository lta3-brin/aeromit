<template>
  <q-page class="q-pt-xl">
    <q-card>
      <q-bar>
        <q-icon name="edit" />
        <div>Kelola Data Pengguna</div>
      </q-bar>

      <q-card-section>
        <q-form
          @submit="onSubmit"
          @reset="onReset"
          class="q-gutter-md"
        >
          <q-input
            type="text"
            filled
            v-model="nama"
            label="Nama Lengkap"
            hint=""
            autocomplete="off"
            lazy-rules
            :rules="[
              val => val && val.length > 0 || 'Nama harus diisi',
            ]"
          />

          <q-input
            filled
            type="text"
            v-model="instansi"
            label="Instansi/Lembaga/Kampus"
            hint=""
            lazy-rules
            :rules="[
              val => val !== null && val !== '' || 'Instansi dan sejenisnya harus diisi',
            ]"
          />

          <div>
            <q-btn label="Simpan" type="submit" color="primary"/>
            <q-btn label="Kosongkan" type="reset" color="primary" flat class="q-ml-sm" />
          </div>
        </q-form>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script src="./profil.js" />
