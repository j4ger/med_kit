<template>
  <w-app id="app">
    <header class="w-flex align-center wrap py0">
      <w-drawer
        v-model="showAccount"
        push-content
        :overlay-opacity="0.5"
        width="160px"
        class="my0 fill-height"
      >
        <template #pushable>
          <w-flex justify-space-between>
            <w-button class="ma1" outline xl @click="showNav = !showNav">
              <w-icon class="maa">{{ menuIcon }}</w-icon>
            </w-button>
            <p class="title2 mya ml3">{{ $route.name }}</p>
            <w-button
              class="ma1"
              outline
              xl
              @click="showAccount = !showAccount"
            >
              <w-icon class="maa">mdi mdi-account</w-icon>
            </w-button>
          </w-flex>
        </template>
        <w-flex class="grow wrap justify-space-between align-center">
          <span class="text-bold mx1">
            {{ loggedIn ? "已登录：" : "未登录：" }}
          </span>
          <w-button
            v-show="!loggedIn"
            bg-color="primary"
            class="grow"
            @click="showDialog = true"
            >登录</w-button
          >
          <p v-show="loggedIn" class="title4 mr4">{{ username }}</p>
        </w-flex>
      </w-drawer>
    </header>
    <w-drawer
      v-model="showNav"
      push-content
      :overlay-opacity="0.5"
      left
      width="160px"
    >
      <template #pushable>
        <div class="w-flex column align-center justify-center px4 pt2">
          <w-flex class="lg6 xs12 column">
            <router-view></router-view>
          </w-flex>
        </div>
      </template>

      <div class="ma2">
        <w-list :items="navItems" nav color="primary">
          <template #item="{ item }">
            <w-flex class="my2">
              <span class="title5">{{ item.label }}</span>
              <div class="spacer"></div>
              <w-icon md>{{ item.icon }}</w-icon>
            </w-flex>
          </template>
        </w-list>
      </div>
    </w-drawer>
    <footer>
      <w-flex justify-center> 2021 j4ger </w-flex>
    </footer>
    <w-dialog v-model="showDialog" width="300px">
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
      <template #title>
        <w-icon class="mr2">mdi mdi-login</w-icon>登录
      </template>
      <w-form v-model="formValidate">
        <w-input
          label="账户"
          label-position="left"
          outline
          inner-icon-left="mdi mdi-account"
          class="mb2"
          :validators="[validators.required]"
          v-model="username"
        >
        </w-input>
        <w-input
          class="mb2"
          label="密码"
          type="password"
          label-position="left"
          outline
          inner-icon-left="mdi mdi-lock"
          :validators="[validators.required]"
          v-model="password"
        >
        </w-input>
        <w-button type="submit" class="fill-width" @click="login"
          >登录</w-button
        >
      </w-form>
    </w-dialog>
  </w-app>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import {
  STDJSONResponse,
  VerifyData,
  LoginData,
  RegisterData,
} from "./Response";
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
  name: "App",
  data() {
    return {
      showNav: false,
      showAccount: false,
      navItems: [
        {
          label: "产品初始化",
          id: "label",
          icon: "mdi mdi-label-multiple",
          route: "/init",
        },
        {
          label: "信息查询",
          id: "info",
          icon: "mdi mdi-book-information-variant",
          route: "/query",
        },
      ],
      loggedIn: false,
      username: "",
      password: "",
      showNotification: false,
      notificationContent: "",
      showDialog: false,
      validators: {
        required: (value) => !!value || "此项必填！",
      },
      formValidate: false,
    };
  },
  computed: {
    menuIcon(): String {
      return this.showNav ? "mdi mdi-menu-open" : "mdi mdi-menu";
    },
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
      if (this.formValidate) {
        let hasher = new Hashes.SHA256();
        let pwhashed = hasher.hex_hmac("SWEETENING", this.password);
        axios
          .post<STDJSONResponse<LoginData>>(
            "http://localhost:1146/login",
            new LoginRequest(this.username, pwhashed)
          )
          .then((response) => {
            if (response.data.success) {
              this.loggedIn = true;
              this.showDialog = false;
            } else {
              this.notificationContent = response.data.errmsg;
              this.showNotification = true;
            }
          });
      }
    },
  },
});
</script>

<style>
header {
  max-height: 50px;
  border: 1px solid rgba(0, 0, 0, 0.07);
}
</style>
