<template>
  <div v-if="current">
    <pre>{{current}}</pre>
  </div>

</template>

<script lang="ts">
import usePersonStore from 'src/stores/person';
import { defineComponent } from 'vue';
export default defineComponent({
  name: "PersonalInformation",
  components: {},
    setup(){
      const store = usePersonStore();
      store.fetchCurrent();
      return store;
  }
})
</script>
