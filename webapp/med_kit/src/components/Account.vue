<template>
  <w-notification
    v-model="showNotification"
    warning
    timeout="0"
    plain
    round
    shadow
    top
    center
    transition="slide-fade-up"
  >
    {{ notificationContent
    }}<w-button text @click="showNotification = false">
      <w-icon class="maa">mdi mdi-close-circle</w-icon>
    </w-button>
  </w-notification>
  <w-flex class="grow wrap justify-space-between align-center">
    <span class="text-bold mx1">
      {{ loggedIn ? "已登录：" : "未登录：" }}
    </span>
    <w-button
      w-if="!loggedIn"
      bg-color="primary"
      class="grow"
      @click="showDialog = true"
      >登录</w-button
    >
    <p v-if="loggedIn" class="title4">{{ username }}</p>
  </w-flex>
  <w-dialog v-model="showDialog">
    <template #title> <w-icon class="mr2">mdi mdi-login</w-icon>登录 </template>
  </w-dialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import {
  STDJSONResponse,
  VerifyData,
  LoginData,
  RegisterData,
} from "../Response";
import axios from "axios";
import Hashes from "jshashes";

class LoginRequest {
  username: string;
  pwhashed: string;

  constructor(username: string, pwhashed: string) {
    this.username = username;
    this.pwhashed = pwhashed;
  }
}

export default defineComponent({
  name: "Account",
  data() {
    return {
      loggedIn: false,
      username: "",
      password: "",
      showNotification: false,
      notificationContent: "",
      showDialog: false,
    };
  },
  mounted() {
    axios
      .get<STDJSONResponse<VerifyData>>("http://localhost:1146/verify", {
        withCredentials: true,
      })
      .then((response) => {
        if (response.data.success) {
          this.username = response.data.data.username;
          this.loggedIn = true;
        }
      });
  },
  methods: {
    login() {
      let hasher = new Hashes.SHA256();
      let pwhashed = hasher.hex_hmac("SWEETENING", this.password);
      console.log(hasher.hex_hmac("SWEETENING", "bagley"));
      axios
        .post<STDJSONResponse<LoginData>>(
          "http://localhost:1146/login",
          new LoginRequest(this.username, pwhashed)
        )
        .then((response) => {
          if (response.data.success) {
            this.loggedIn = true;
          } else {
            this.notificationContent = response.data.errmsg;
            this.showNotification = true;
          }
        });
    },
  },
});
</script>

<style></style>
