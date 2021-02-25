<template>
  <w-app id="app">
    <header>
      <w-flex justify-start>
        <w-button class="ma1" text xl @click="showNav = !showNav">
          <w-icon class="maa">{{ menuIcon }}</w-icon>
        </w-button>
        <p class="title2 mya ml3">测试系统</p>
      </w-flex>
    </header>
    <w-drawer
      v-model="showNav"
      push-content
      :overlay-opacity="0.15"
      left
      width="160px"
    >
      <template #pushable>
        <div class="w-flex column align-center justify-center pa6">
          <w-flex class="sm6 xs12 column">
            <div class="row mb8">
              <init-prod></init-prod>
            </div>
            <div class="row">
              <fetch-profile uuid="1145141919"></fetch-profile>
            </div>
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
  </w-app>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import InitProd from "./components/InitProduct.vue";
//TODO: remove this
import SubmitProfile from "./components/SubmitProfile.vue";
import FetchProfile from "./components/FetchProfile.vue";

export default defineComponent({
  name: "App",
  components: {
    InitProd,
    SubmitProfile,
    FetchProfile,
  },
  data() {
    return {
      showNav: false,
      navItems: [
        {
          label: "标签打印",
          id: "label",
          icon: "mdi mdi-label-multiple",
          route: "/new",
        },
        {
          label: "信息登记",
          id: "info",
          icon: "mdi mdi-book-information-variant",
          route: "/profile",
        },
      ],
    };
  },
  computed: {
    menuIcon(): String {
      return this.showNav ? "mdi mdi-menu-open" : "mdi mdi-menu";
    },
  },
});
</script>

<style scoped>
header {
  min-height: 50px;
  padding: 10px;
  border: 1px solid rgba(0, 0, 0, 0.07);
}
</style>
