<template>
  <w-app id="app">
    <header>
      <w-flex justify-start>
        <w-button class="ma1" text xl @click="showNav = !showNav">
          <w-icon class="maa">{{ menuIcon }}</w-icon>
        </w-button>
        <p class="title2 mya ml3">{{ $route.name }}</p>
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
        <div class="w-flex column align-center justify-center px4 pt2">
          <w-flex class="sm6 xs12 column">
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
  </w-app>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "App",
  data() {
    return {
      showNav: false,
      navItems: [
        {
          label: "标签打印",
          id: "label",
          icon: "mdi mdi-label-multiple",
          route: "/init",
        },
        {
          label: "信息登记",
          id: "info",
          icon: "mdi mdi-book-information-variant",
          route: "/fetch/114",
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

<style>
header {
  min-height: 50px;
  padding: 10px;
  border: 1px solid rgba(0, 0, 0, 0.07);
}
</style>
