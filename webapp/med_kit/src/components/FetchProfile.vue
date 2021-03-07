<template>
  <w-flex class="pl4">
    <w-input label="产品序列号" v-model="uuid" class="mb4">产品序列号</w-input>
    <w-button type="submit" class="mx4" @click="submit">提交</w-button>
  </w-flex>
  <w-alert bg-color="primary" color="white" class="px4" v-show="uuidValid">
    正在{{ fetched ? "处理" : "获取" }}序列号为{{ uuid }}的产品
  </w-alert>
  <component
    v-if="fetched"
    :is="innerComponent"
    v-bind="{ uuid: uuid }"
    ref="innerComponentRef"
  ></component>
  <w-notification
    v-model="showNotification"
    warning
    timeout="0"
    plain
    round
    shadow
    bottom
    center
    transition="slide-fade-up"
  >
    {{ notificationContent
    }}<w-button text @click="showNotification = false">
      <w-icon class="maa">mdi mdi-close-circle</w-icon>
    </w-button>
  </w-notification>
</template>

<script lang="ts">
import { defineComponent, Ref, ref } from "vue";
import axios from "axios";
import SubmitProfile from "./SubmitProfile.vue";
import ShowProfile from "./ShowProfile.vue";
import { STDJSONResponse, QueryData } from "../Response";
import Profile from "../Profile";

export default defineComponent({
  name: "FetchProfile",
  components: { SubmitProfile, ShowProfile },
  data() {
    return {
      fetched: false,
      innerComponent: "",
      showNotification: false,
      notificationContent: "",
      uuidValid: false,
      uuid: "",
    };
  },
  setup() {
    let innerComponentRef = ref(null);
    return { innerComponentRef };
  },
  methods: {
    submit() {
      // TODO: uuid校验
      if (this.uuid != "") {
        this.uuidValid = true;
        axios
          .get<STDJSONResponse<QueryData>>(
            "http://localhost:1146/query/" + this.uuid
          )
          .then((response) => {
            if (response.data.success) {
              if (response.data.data.exist) {
                if (!response.data.data.init) {
                  this.innerComponent = "SubmitProfile";
                } else {
                  this.innerComponent = "ShowProfile";
                  this.innerComponentRef.updateData();
                }
                this.fetched = true;
              } else {
                this.notificationContent = "产品序列号不存在！";
                this.showNotification = true;
              }
            } else {
              this.notificationContent = "服务器错误！";
              this.showNotification = true;
            }
          });
      }
    },
  },
  mounted() {
    if (this.$route.params.uuid != undefined) {
      this.uuid = this.$route.params.uuid;
      this.uuidValid = true;
      //TODO: 请求地址
      axios
        .get<STDJSONResponse<QueryData>>(
          "http://localhost:1146/query/" + this.$route.params.uuid
        )
        .then((response) => {
          if (response.data.success) {
            if (response.data.data.exist) {
              if (!response.data.data.init) {
                this.innerComponent = "SubmitProfile";
              } else {
                this.innerComponent = "ShowProfile";
                this.innerComponentRef.updateData();
              }
              this.fetched = true;
            } else {
              this.notificationContent = "产品序列号不存在！";
              this.showNotification = true;
            }
          } else {
            this.notificationContent = "服务器错误！";
            this.showNotification = true;
          }
        });
    }
  },
});
</script>

<style></style>
