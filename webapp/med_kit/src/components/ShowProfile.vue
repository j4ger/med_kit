<template>
  <w-flex class="column align-center justify-center">
    <w-flex class="column lg4 xs12">
      <w-list :items="Object.entries(profile).length - 1" hover>
        <template #item="{ index }">
          <w-flex basis-zero class="row">
            <span class="title5 maa grow">{{
              getDisplayName(Object.keys(profile)[index])
            }}</span>
            <span class="body maa grow">{{
              Object.values(profile)[index]
            }}</span>
          </w-flex>
        </template>
      </w-list>
    </w-flex>
  </w-flex>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { getDisplayName } from "../Profile";
export default defineComponent({
  name: "SubmitProfile",
  props: ["uuid", "profile"],
  data() {
    return {
      getDisplayName,
    };
  },
});
</script>

<style></style>
