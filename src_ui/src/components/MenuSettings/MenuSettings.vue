<template>
  <q-menu>
    <q-list>
      <q-item clickable v-close-popup @click="goToPage('profil')">
        <q-item-section avatar>
          <q-icon name="perm_identity" />
        </q-item-section>
        <q-item-section>Profil</q-item-section>
      </q-item>

      <q-separator />

      <q-item clickable v-close-popup @click="goToPage('masuk')" v-if="!isSignIn">
        <q-item-section avatar>
          <q-icon name="login" />
        </q-item-section>
        <q-item-section>Masuk</q-item-section>
      </q-item>

      <q-item clickable v-close-popup @click="keluar" v-else>
        <q-item-section avatar>
          <q-icon name="exit_to_app" />
        </q-item-section>
        <q-item-section>Keluar</q-item-section>
      </q-item>
    </q-list>
  </q-menu>
</template>

<script src="./MenuSettings.js" />
