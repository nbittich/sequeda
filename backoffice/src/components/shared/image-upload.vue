<script lang="ts">
import { computed, defineComponent, Ref, ref, toRefs } from 'vue';
import { QFile } from 'quasar';
import useUploadStore from 'src/stores/uploads';
const uploadStore = useUploadStore();

export default defineComponent({
  name: 'ImageUpload',
  props: {
    modelValue: {
      type: Object,
      default: () => ({}) as File,
    },
    pictureId: {
      type: String,
      default: () => null,
    },
  },
  async setup(props, context) {
    const { pictureId } = toRefs(props);
    const picture = computed({
      get: () => props.modelValue,
      set: (value) => context.emit('update:modelValue', value),
    });

    // for some reason picture above is no longer reactive with computed in this case (!)
    // workaround is to wrap it into a ref
    // maybe there's a better way, not willing to read the manual to find out.
    const pictureFile = ref(picture.value as File);

    const fileRef = ref() as Ref<QFile>;
    const pictureUrl = ref(null as unknown as string);

    const pictureUrlChange = async () => {
      if (pictureFile.value) {
        if (pictureFile.value.type?.startsWith('image')) {
          pictureUrl.value = URL.createObjectURL(pictureFile.value);
        } else {
          pictureUrl.value = 'images/question.png';
        }
        context.emit('update:modelValue', pictureFile.value);
      } else {
        if (pictureId.value) {
          const pictureMetadata = await uploadStore.getMetadata(
            pictureId.value,
          );

          pictureUrl.value = uploadStore.getDownloadUrl(
            pictureMetadata.thumbnailId,
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
  />
</template>
