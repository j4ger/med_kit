<template>
  <w-card title="用户信息">
    <w-form v-model="formValidate" @submit="submit">
      <w-input
        label="姓名"
        :validators="[validators.required]"
        v-model="name"
        class="mb4"
        >姓名<w-tag class="mt-1 pa0" color="error">必填</w-tag></w-input
      >
      <w-radios
        v-model="gender"
        :items="genderOptions"
        :validators="[validators.required]"
        class="mb4"
        inline
      ></w-radios>
      <w-input
        label="年龄"
        v-model="age"
        class="mb4"
        :validators="[validators.age]"
        >年龄<w-tag class="mt-1 pa0" color="error">必填</w-tag></w-input
      >
      <w-input
        label="手机号码"
        v-model="phone"
        :validators="[validators.phone]"
        class="mb4"
        >年龄<w-tag class="mt-1 pa0" color="error">必填</w-tag></w-input
      >
      <w-input label="电子邮件地址" v-model="email" class="mb4"></w-input>
      <address-picker v-model="address" class="mb4"></address-picker>
      <w-flex>
        <div id="dateInput" class="sm6 mr2">
          <span id="timeInput" class="primary"
            >采样日期<w-tag class="mt-1 pa0" color="error">必填</w-tag></span
          >
          <vue-cal
            :time="false"
            active-view="month"
            :disable-views="['week', 'day']"
            locale="zh-cn"
            todayButton
            class="vuecal--date-picker mb4"
            :selectedDate="selectedDate"
            @cell-click="selectedDate = $event"
          ></vue-cal>
        </div>
        <div class="sm6 ml10">
          <w-select
            :items="
              [...Array(24).keys()].map((each) => {
                return { label: each };
              })
            "
            label="采样时间（时）"
            v-model="selectedHour"
            class="mb4"
            :validators="[validators.required]"
            >采样时间（小时）<w-tag class="mt-1 pa0" color="error">必填</w-tag>
          </w-select>
        </div>
      </w-flex>

      <w-flex class="basis-zero mt4">
        <w-button type="submit" class="mx4">提交</w-button>
        <w-button bg-color="warning" type="reset" class="mx4">重置</w-button>
      </w-flex>
    </w-form>
  </w-card>
  <w-notification
    v-model="showNotification"
    :success="updateSuccess"
    :warning="!updateSuccess"
    timeout="0"
    plain
    round
    shadow
    bottom
    center
    transition="slide-fade-up"
  >
    {{ updateSuccess ? "提交成功" : "该序列号已被使用"
    }}<w-button text @click="showNotification = false"
      ><w-icon class="maa">mdi mdi-close-circle</w-icon></w-button
    ></w-notification
  >
</template>

<script lang="ts">
import { defineComponent } from "vue";
import axios from "axios";
import Profile from "../Profile";
import { STDJSONResponse, UpdateData } from "../Response";
import VueCal from "vue-cal";
import "vue-cal/dist/i18n/zh-cn.js";
import "vue-cal/dist/vuecal.css";
import AddressPicker from "./AddressPicker.vue";

export default defineComponent({
  name: "SubmitProfile",
  props: ["uuid"],
  components: { AddressPicker, VueCal },
  data() {
    return {
      name: "",
      gender: "",
      selectedDate: new Date(),
      selectedHour: new Date().getHours(),
      phone: "",
      address: "",
      age: null,
      email: "",
      hospital: "",
      validators: {
        required: (value: any) => !!value || "此项为必填！",
        phone: (value: any) =>
          /^1[0-9]{10}$/.test(value) || "请正确填写手机号码！",
        age: (value: any) =>
          /^(?:[1-9][0-9]?|1[01234][0-9]|150)$/.test(value) ||
          "请正确填写年龄！",
      },
      formValidate: false,
      updateSuccess: false,
      showNotification: false,
      genderOptions: [
        { label: "男", value: "男" },
        { label: "女", value: "女" },
      ],
    };
  },
  computed: {
    time(): string {
      return (
        this.selectedDate.format("YYYY年MM月DD日") + this.selectedHour + "时"
      );
    },
  },
  methods: {
    submit() {
      if (this.formValidate) {
        //TODO: 请求地址
        axios
          .post<STDJSONResponse<UpdateData>>(
            "http://localhost:1146/submit",
            new Profile(
              this.uuid,
              this.name,
              this.gender,
              this.time,
              this.phone,
              this.address,
              this.age,
              this.email,
              this.hospital
            )
          )
          .then((response) => {
            if (response.data.success) {
              if (
                response.data.data.init == false &&
                response.data.data.exist &&
                response.data.data.updated_count == 1
              ) {
                this.updateSuccess = true;
              }
              this.showNotification = true;
            }
          });
      }
    },
  },
});
</script>

<style scoped>
#timeInput {
  font-size: 13px;
}
#dateInput {
  max-width: 500px;
  max-height: 500px;
}
</style>
