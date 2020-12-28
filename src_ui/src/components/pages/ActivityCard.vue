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
      <q-btn flat round icon="event" @click="openDialog = true" />
      <q-btn flat color="primary">
        Gabung
      </q-btn>
    </q-card-actions>

    <q-dialog v-model="openDialog">
      <q-card>
        <q-card-section>
          <div class="text-h6">
            {{ data['nama'] }}
          </div>
        </q-card-section>

        <q-separator />

        <q-card-section style="max-height: 50vh" class="scroll">
          <q-list>
            <q-item>
              <q-item-section>
                <q-item-label>Waktu Pelaksanaan</q-item-label>
                <q-item-label caption lines="2">
                  {{ data['kapan'] | format }}
                </q-item-label>
              </q-item-section>

              <q-item-section side center>
                <q-icon name="alarm" color="cyan" />
              </q-item-section>
            </q-item>

            <q-separator spaced inset />

            <q-item>
              <q-item-section>
                <q-item-label>Moderator</q-item-label>
                <q-item-label caption>
                  {{ data['moderator'] }}
                </q-item-label>
              </q-item-section>

              <q-item-section side center>
                <q-icon name="mic" color="cyan" />
              </q-item-section>
            </q-item>

            <q-separator spaced inset />

            <div v-for="(val, k) in data['pembicara']" :key="val['nama']">
              <q-item>
                <q-item-section>
                  <q-item-label v-if="data['pembicara'].length > 1">Pembicara {{ k + 1 }}</q-item-label>
                  <q-item-label v-else>Pembicara</q-item-label>
                  <q-item-label caption>
                    {{ val['nama'] }} - {{ val['judul'] }}
                  </q-item-label>
                </q-item-section>

                <q-item-section side center>
                  <q-icon name="record_voice_over" color="cyan" />
                </q-item-section>
              </q-item>

              <q-separator spaced inset />
            </div>

            <q-item>
              <q-item-section>
                <q-item-label>Tautan</q-item-label>
                <q-item-label caption>
                  <a :href="data['tautanVideo']"
                     rel="noopener noreferrer"
                     target="_blank"
                  >
                    {{ data['tautanVideo'] }}
                  </a>
                </q-item-label>
              </q-item-section>

              <q-item-section side center>
                <q-icon name="link" color="cyan" />
              </q-item-section>
            </q-item>
          </q-list>
        </q-card-section>
      </q-card>
    </q-dialog>
  </q-card>
</template>

<script>
import {date} from 'quasar'

export default {
  name: 'ActivityPageComponent',
  data() {
    return {
      openDialog: false
    }
  },
  props: {
    data: Object
  },
  filters: {
    format(val) {
      return date.formatDate(val, 'DD MMMM YYYY, HH:mm')
    }
  }
}
</script>
