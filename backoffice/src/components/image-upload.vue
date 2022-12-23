<script lang="ts">
import { defineComponent, Ref, ref, toRefs } from 'vue';
import { QFile } from 'quasar';
import useUploadStore from 'src/stores/uploads';
const uploadStore = useUploadStore();

export default defineComponent({
  name: 'ImageUpload',
  props: {
    modelValue: {
      type: Object,
      default: () => ({} as File),
    },
    pictureId: {
      type: String,
      default: () => null,
    },
  },
  async setup(props, context) {
    const { pictureId } = toRefs(props);
    const pictureFile = ref(null as unknown as File);

    const fileRef = ref() as Ref<QFile>;
    const pictureUrl = ref(null as unknown as string);

    const pictureUrlChange = async () => {
      if (pictureFile.value) {
        pictureUrl.value = URL.createObjectURL(pictureFile.value);
        context.emit('update:modelValue', pictureFile);
      } else {
        if (pictureId.value) {
          const pictureMetadata = await uploadStore.getMetadata(
            pictureId.value
          );

          pictureUrl.value = uploadStore.getDownloadUrl(
            pictureMetadata.thumbnailId
          );
        } else {
          pictureUrl.value = 'images/unknown.png';
        }
      }
    };
    await pictureUrlChange();
    return {
      fileRef,
      pictureFile,
      pictureUrl,
      pictureUrlChange,
      selectFile() {
        fileRef.value.pickFiles();
      },
    };
  },
});
</script>
<template>
  <q-img
    class="border-fluid rounded-borders"
    :src="pictureUrl"
    spinner-color="white"
    @click="selectFile()"
    style="height: 140px; max-width: 150px"
  />
  <q-file
    ref="fileRef"
    style="display: none"
    v-model="pictureFile"
    @update:model-value="pictureUrlChange()"
    accept="image/*"
  />
</template>
