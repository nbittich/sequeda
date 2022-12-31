<script lang="ts">
import { computed, defineComponent, ref } from 'vue';
import { Remark } from 'src/models/orgs';
export default defineComponent({
    name: 'RemarkForm',
    props: {
        modelValue: {
            type: Object,
            default: () => [] as Remark[]
        },
        title: {
            type: String,
            default: 'Remarks'
        },

    },
    async setup(props, context) {
        const remarksComputed = computed({
            get: () => props.modelValue,
            set: (value) => context.emit('update:modelValue', value)
        });
        const remarks = ref(remarksComputed);

        return {
            remarks
        }
    },
    methods: {
        addRemark() {
            this.remarks.push({} as Remark);
        }
    },
})
</script>
<template>
    <q-card>
        <q-card-section>
            <div class="row justify-between">
                <div class="text-h6">{{ title }}</div>
                <q-btn round icon="add" color="primary" @click="addRemark()" />
            </div>
        </q-card-section>

        <q-card-section>
            <template v-for="(remark, index) in remarks" :key="index">
                <div class="row q-mb-xs-none q-mb-md-xs">
                    <div class="col-12">
                        <q-input v-model="remark.message" dense outlined label="Message" autogrow />
                    </div>
                </div>
            </template>
        </q-card-section>
    </q-card>
</template>
