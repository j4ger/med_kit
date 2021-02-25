<template>
  <div class="row">
    <w-input v-model="uuid">序列号</w-input>
  </div>
  <div class="row">
    <w-flex>
      <w-button class="mt4 grow" bg-color="primary" @click="init"
        >提交</w-button
      >
    </w-flex>
  </div>
  <w-flex v-if="fetched" class="mt4" justify-center>
    <div class="row">
      <qrcode-vue :value="target_url"></qrcode-vue>
    </div>
    <div class="row">
      <a :href="target_url">{{ uuid }}</a>
    </div>
  </w-flex>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import QrcodeVue from "qrcode.vue";
import axios from "axios";
import { STDJSONResponse, InitData } from "../Response";

export default defineComponent({
  name: "InitProd",
  components: { QrcodeVue },
  data() {
    return {
      uuid: "",
      fetched: false,
      duplicate: false,
    };
  },
  computed: {
    target_url(): string {
      return "/fetch/" + this.uuid;
    },
  },
  methods: {
    init() {
      //TODO: 请求地址
      axios
        .get<STDJSONResponse<InitData>>(
          "http://localhost:1146/init/" + this.uuid
        )
        .then((response) => {
          if (response.data.success) {
            this.fetched = true;
            this.duplicate = response.data.data.exist;
          } else {
            //TODO: 错误处理
            alert("err");
          }
        });
    },
  },
});
</script>

<style></style>
