<template>
  <TransitionRoot
    as="template"
    :show="true"
    :style="{ 'z-index': props.zHeight }"
  >
    <Dialog class="relative z-50" @close="emit('event', 'cancel')">
      <TransitionChild
        as="template"
        enter="ease-out duration-300"
        enter-from="opacity-0"
        enter-to="opacity-100"
        leave="ease-in duration-200"
        leave-from="opacity-100"
        leave-to="opacity-0"
      >
        <div
          class="fixed inset-0 bg-zinc-950 bg-opacity-75 transition-opacity"
        />
      </TransitionChild>

      <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
        <div
          class="flex min-h-full items-start justify-center p-4 text-center sm:items-center sm:p-0"
        >
          <TransitionChild
            as="template"
            enter="ease-out duration-300"
            enter-from="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
            enter-to="opacity-100 translate-y-0 sm:scale-100"
            leave="ease-in duration-200"
            leave-from="opacity-100 translate-y-0 sm:scale-100"
            leave-to="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
          >
            <div
              class="relative transform rounded-lg bg-zinc-900 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg"
            >
              <div class="px-4 pb-4 pt-5 space-y-4 sm:p-6 sm:pb-4">
                <div class="sm:flex sm:items-start">
                  <div class="mt-3 text-center sm:mt-0 sm:text-left">
                    <DialogTitle
                      as="h3"
                      class="text-base font-semibold text-zinc-100"
                      >{{ props.data.title }}
                    </DialogTitle>
                    <div class="mt-2">
                      <p class="text-sm text-zinc-400">
                        {{ props.data.description }}
                      </p>
                    </div>
                    <form @submit.prevent="() => emit('event', 'submit', v)">
                      <input
                        v-model="v"
                        type="text"
                        :placeholder="$t('library.collection.namePlaceholder')"
                        class="block w-full rounded-md border-0 bg-zinc-800 py-1.5 text-white shadow-sm ring-1 ring-inset ring-zinc-700 placeholder:text-zinc-400 focus:ring-2 focus:ring-inset focus:ring-blue-600 sm:text-sm sm:leading-6"
                      />
                      <button class="hidden" type="submit" />
                    </form>
                  </div>
                </div>
              </div>
              <div
                class="rounded-b-lg bg-zinc-800 px-4 py-3 sm:flex sm:gap-x-2 sm:flex-row-reverse sm:px-6"
              >
                <LoadingButton
                  :loading="props.loading"
                  @click="emit('event', 'submit', v)"
                  type="submit"
                  class="w-full sm:w-fit"
                >
                  {{ props.data.buttonText ?? "Confirm" }}
                </LoadingButton>
                <button
                  type="button"
                  class="mt-3 inline-flex w-full justify-center rounded-md bg-zinc-800 px-3 py-2 text-sm font-semibold text-zinc-100 shadow-sm ring-1 ring-inset ring-zinc-700 hover:bg-zinc-900 sm:mt-0 sm:w-auto"
                  @click="emit('event', 'cancel')"
                  ref="cancelButtonRef"
                >
                  Cancel
                </button>
              </div>
            </div>
          </TransitionChild>
        </div>
      </div>
    </Dialog>
  </TransitionRoot>
</template>

<script setup lang="ts">
import {
  Dialog,
  DialogTitle,
  TransitionChild,
  TransitionRoot,
} from "@headlessui/vue";
import type {
  ModalDatas,
  ModalEvents,
  ModalType,
} from "../composables/modal-stack";

const props = defineProps<{
  zHeight: number;
  loading: boolean;
  data: ModalDatas[ModalType.TextInput];
}>();
const emit = defineEmits<{
  (e: "event", v: ModalEvents[ModalType.TextInput], s?: string): void;
}>();

const v = ref(props.data.dft || "");
</script>
