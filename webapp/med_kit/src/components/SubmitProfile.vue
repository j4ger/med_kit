<template>
  <w-card title="用户信息">
    <w-form v-model="formValidate" @submit="submit">
      <w-input
        label="姓名"
        :validators="[validators.required]"
        v-model="name"
        class="mb4"
      ></w-input>
      <w-flex class="basis-zero">
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
export default defineComponent({
  name: "SubmitProfile",
  props: ["uuid"],
  data() {
    return {
      name: "",
      validators: {
        required: (value: any) => !!value || "此项为必填！",
      },
      formValidate: false,
      updateSuccess: false,
      showNotification: false,
    };
  },
  methods: {
    submit() {
      if (this.formValidate) {
        //TODO: 请求地址
        axios
          .post<STDJSONResponse<UpdateData>>(
            "http://localhost:1146/submit",
            new Profile(this.uuid, this.name)
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

<style></style>
