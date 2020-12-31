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

      <q-item clickable v-close-popup @click="goToPage('login')">
        <q-item-section avatar>
          <q-icon name="login" />
        </q-item-section>
        <q-item-section>Masuk</q-item-section>
      </q-item>
    </q-list>
  </q-menu>
</template>

<script src="./MenuSettings.js" />
