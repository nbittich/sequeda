<template>
    <q-layout view="hHh Lpr lff" class="shadow-2">
      <q-header elevated class="bg-teal">
        <q-toolbar>
          <q-btn flat @click="drawer = !drawer" round dense icon="menu" />
          <q-toolbar-title>Sequeda</q-toolbar-title>
        </q-toolbar>
      </q-header>

      <q-drawer
        v-model="drawer"
        show-if-above
        :mini="!drawer || miniState"
        @click.capture="drawerClick"
        :width="200"
        :breakpoint="200"
        bordered
        class="bg-grey-3"
      >
        <template v-slot:mini>
          <q-scroll-area class="fit mini-slot cursor-pointer">
            <div class="q-py-lg">
              <div class="column items-start">
                <q-icon name="inbox" color="blue" class="mini-icon" />
                <q-icon name="star" color="orange" class="mini-icon" />
                <q-icon name="send" color="purple" class="mini-icon" />
                <q-icon name="drafts" color="teal" class="mini-icon" />
              </div>
            </div>
          </q-scroll-area>
        </template>

        <q-scroll-area class="fit">
          <q-list padding>
            <q-item clickable v-ripple>
              <q-item-section> Inbox </q-item-section>
            </q-item>

            <q-item active clickable v-ripple>
              <q-item-section> Star </q-item-section>
            </q-item>

            <q-item clickable v-ripple>
              <q-item-section> Send </q-item-section>
            </q-item>

            <q-item clickable v-ripple>
              <q-item-section> Drafts </q-item-section>
            </q-item>
          </q-list>
        </q-scroll-area>

        <!--
          in this case, we use a button (can be anything)
          so that user can switch back
          to mini-mode
        -->
        <div
          class="q-mini-drawer-hide absolute"
          style="top: 15px; right: -17px"
        >
          <q-btn
            dense
            round
            unelevated
            color="accent"
            icon="chevron_left"
            @click="miniState = true"
          />
        </div>
      </q-drawer>

      <q-page-container>
        <q-page class="q-px-lg q-py-md">
          <router-view />
        </q-page>
      </q-page-container>
    </q-layout>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';

export default defineComponent({
  name: 'MainLayout',

  components: {},

  setup() {
    const miniState = ref(true);

    return {
      drawer: ref(false),
      miniState,
      drawerClick(e: Event) {
        // if in "mini" state and user
        // click on drawer, we switch it to "normal" mode
        if (miniState.value) {
          miniState.value = false;

          // notice we have registered an event with capture flag;
          // we need to stop further propagation as this click is
          // intended for switching drawer to "normal" mode only
          e.stopPropagation();
        }
      },
    };
  },
});
</script>
<style lang="sass" scoped>
.mini-slot
  transition: background-color .28s
  &:hover
    background-color: rgba(0, 0, 0, .04)

.mini-icon
  font-size: 1.718em
  padding: 2px 16px

  & + &
    margin-top: 18px
</style>
