<script lang="ts">
import { computed, defineComponent } from 'vue';
import { BankAccount } from 'src/models/bank-account';

export default defineComponent({
  name: 'BankAccountForm',
  props: {
    title: {
      type: String,
      default: () => 'Bank Account',
    },
    deletable: {
      type: Boolean,
      default: () => false,
    },
    modelValue: {
      type: Object,
      default: () => ({} as BankAccount),
    },
  },
  emits: ['update:modelValue', 'deleted'],
  async setup(props, context) {
    const bankAccount = computed({
      get: () => props.modelValue,
      set: (value) => context.emit('update:modelValue', value),
    });

    if (!bankAccount.value) {
      bankAccount.value = {} as BankAccount;
    }

    return {
      bankAccount,
    };
  },
  methods: {
    deleteBankAccount() {
      if (this.deletable) {
        this.$emit('deleted');
      }
    },
  },
});
</script>

<template>
  <q-card>
    <q-card-section>
      <div class="row justify-between">
        <div class="text-h6">{{ title }}</div>
        <q-btn
          round
          icon="delete"
          color="red"
          v-if="deletable"
          @click="deleteBankAccount"
        />
      </div>
    </q-card-section>
    <q-card-section class="q-mt-xs-sm q-mt-md-none q-pt-none">
      <div class="row q-mb-xs-none q-mb-md-xs">
        <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-input
            type="text"
            class="q-mr-md-xs"
            dense
            outlined
            v-model="bankAccount.number"
            label="IBAN"
          />
        </div>
        <div class="col-lg-6 col-12 q-mb-xs-sm q-mb-lg-none">
          <q-input
            type="text"
            class="q-mr-md-xs"
            dense
            outlined
            v-model="bankAccount.bic"
            label="BIC"
          />
        </div>
      </div>
    </q-card-section>
  </q-card>
</template>
