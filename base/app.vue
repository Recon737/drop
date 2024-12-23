<template>
  <ModalStack />
</template>

<script setup lang="ts">
onMounted(() => {
  createModal(
    ModalType.Confirmation,
    {
      title: "Are you sure?",
      description:
        "Are you sure you want to complete this action? It may have unintended side effects.",
    },
    async (event, close) => {
      switch (event) {
        case "confirm":
          await new Promise((r) => setTimeout(r, 3000));
          close();
          break;
        case "cancel":
          close();
          break;
      }
    }
  );
});
</script>
