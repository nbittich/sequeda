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
      <q-scroll-area class="fit">
        <q-list padding class="menu-list">
          <q-item clickable v-ripple to="/personal-info">
            <q-item-section avatar>
              <q-icon name="perm_identity" />
            </q-item-section>
            <q-item-section> Profile </q-item-section>
          </q-item>
          <q-expansion-item
            expand-separator
            icon="corporate_fare"
            label="Organization"
          >
            <q-item clickable v-ripple to="/org">
              <q-item-section avatar>
                <q-icon name="store" />
              </q-item-section>
              <q-item-section> General </q-item-section>
            </q-item>
            <q-item clickable v-ripple to="/org/positions">
              <q-item-section avatar>
                <q-icon name="school" />
              </q-item-section>
              <q-item-section> Positions </q-item-section>
            </q-item>
            <q-item clickable v-ripple to="/org/members">
              <q-item-section avatar>
                <q-icon name="badge" />
              </q-item-section>
              <q-item-section> Members </q-item-section>
            </q-item>
            <q-item clickable v-ripple to="/org/customers">
              <q-item-section avatar>
                <q-icon name="recent_actors" />
              </q-item-section>
              <q-item-section> Customers </q-item-section>
            </q-item>
          </q-expansion-item>

          <q-item clickable v-ripple to="/product">
            <q-item-section avatar>
              <q-icon name="local_mall" />
            </q-item-section>
            <q-item-section> Products </q-item-section>
          </q-item>
          <q-item clickable v-ripple to="/admin">
            <q-item-section avatar>
              <q-icon name="admin_panel_settings" />
            </q-item-section>
            <q-item-section> Admin </q-item-section>
          </q-item>
        </q-list>
      </q-scroll-area>

      <!--
          in this case, we use a button (can be anything)
          so that user can switch back
          to mini-mode
        -->
      <div class="q-mini-drawer-hide absolute" style="top: 15px; right: -17px">
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
      <q-page>
        <Suspense>
          <template #default>
            <router-view
              :key="($route.params.id as string) || $route.query.t?.toString()"
            />
          </template>
          <template #fallback>
            <div>Loading...</div>
          </template>
        </Suspense>
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
  .menu-list .q-item
    border-radius: 0 32px 32px 0
</style>
