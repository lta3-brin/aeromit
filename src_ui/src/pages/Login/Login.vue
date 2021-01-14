<template>
  <q-page class="flex flex-center">
    <q-card class="q-masuk q-mx-md">
      <q-bar>
        <q-icon name="input" />
        <div>Masuk Aeromit</div>
      </q-bar>

      <q-card-section v-if="!showBanner">
        <q-banner rounded inline-actions class="text-white bg-negative">
          {{ pesan }}
          <template v-slot:avatar>
            <q-icon name="info" color="white" size="md" />
          </template>
        </q-banner>
      </q-card-section>

      <q-card-section>
        <q-form
          @submit="onSubmit"
          @reset="onReset"
          class="q-gutter-md"
        >
          <q-input
            type="email"
            filled
            v-model="email"
            label="Email"
            hint=""
            autocomplete="off"
            lazy-rules
            :rules="[
              val => val && val.length > 0 || 'Email harus diisi',
              val => this.validateEmail(val) || 'Harus dalam format email'
            ]"
          />

          <q-input
            filled
            type="password"
            v-model="password"
            label="Password"
            hint=""
            lazy-rules
            :rules="[
              val => val !== null && val !== '' || 'Sandi harus diisi',
              val => val.length >= 6 || 'Minimum sandi 6 karakter'
            ]"
          />

          <div>
            <q-btn label="Masuk" type="submit" color="primary"/>
            <q-btn label="Kosongkan" type="reset" color="primary" flat class="q-ml-sm" />
          </div>
        </q-form>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script src="./login.js" />
