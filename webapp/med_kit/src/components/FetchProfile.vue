<template>
  <w-alert bg-color="primary" color="white" class="px4">
    正在{{ fetched ? "处理" : "获取" }}序列号为{{ uuid }}的产品
  </w-alert>
  <component
    v-if="fetched"
    :is="innerComponent"
    v-bind="{ uuid: uuid, profile: profile }"
  ></component>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import axios from "axios";
import SubmitProfile from "./SubmitProfile.vue";
import ShowProfile from "./ShowProfile.vue";
import { STDJSONResponse, GetData } from "../Response";
import Profile from "../Profile";

export default defineComponent({
  name: "FetchProfile",
  props: ["uuid"],
  components: { SubmitProfile, ShowProfile },
  data() {
    return { fetched: false, innerComponent: "", profile: {} };
  },
  mounted() {
    //TODO: 请求地址
    axios
      .get<STDJSONResponse<GetData<Profile>>>(
        "http://localhost:1146/fetch/" + this.uuid
      )
      .then((response) => {
        if (response.data.success) {
          if (response.data.data.exist) {
            if (!response.data.data.init) {
              this.innerComponent = "SubmitProfile";
            } else {
              this.profile = response.data.data.profile;
              this.innerComponent = "ShowProfile";
            }
            this.fetched = true;
          } else {
            //TODO: 错误处理
            alert("Err");
          }
        } else {
          //TODO: 错误处理
          alert("Err");
        }
      });
  },
});
</script>

<style></style>
