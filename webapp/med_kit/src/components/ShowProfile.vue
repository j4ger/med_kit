<template>
  <w-flex class="column align-center justify-center" v-if="fetched">
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
import { GetData, STDJSONResponse } from "../Response";
import axios from "axios";
export default defineComponent({
  name: "SubmitProfile",
  props: ["uuid"],
  data() {
    return {
      getDisplayName,
      profile: null,
      fetched: false,
    };
  },
  methods: {
    updateData() {
      axios
        .get<STDJSONResponse<GetData>>("http://localhost:1146/get/" + this.uuid)
        .then((response) => {
          if (response.data.success) {
            this.profile = response.data.data.profile;
            this.fetched = true;
          } else {
            //TODO: 错误处理
            alert("err");
          }
        });
    },
  },
  mounted() {
    //TODO: uuid检验
    this.updateData();
  },
});
</script>

<style></style>
