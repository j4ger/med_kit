<template>
  <w-flex wrap grow>
    <div class="row xs12">
      <span id="title" class="primary"
        >地址<w-tag class="mt-1 pa0" color="error">必填</w-tag></span
      >
    </div>
    <div class="row xs12">
      <w-flex wrap>
        <div class="lg4 xs12 my2">
          <w-select
            :items="getSelectItems(100000)"
            v-model="province"
            @input="changeProvince"
            :validators="[validators.required]"
          ></w-select>
        </div>
        <div class="lg4 xs12 my2">
          <w-select
            :items="getSelectItems(province)"
            v-model="city"
            @input="changeCity"
            v-if="province != null"
            :validators="[validators.required]"
          ></w-select>
        </div>
        <div class="lg4 xs12 my2">
          <w-select
            :items="getSelectItems(city)"
            v-model="county"
            @input="changeValue"
            v-if="city != null"
            :validators="[validators.required]"
          ></w-select>
        </div>
        <div class="xs12">
          <w-input
            v-model="detail"
            label="街道地址"
            @input="changeValue"
            v-if="province != null && city != null && county != null"
            :validators="[validators.required]"
            >街道地址<w-tag class="mt-1 pa0" color="error">必填</w-tag></w-input
          >
        </div>
      </w-flex>
    </div>
  </w-flex>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import "../AddressData";
import AddressData from "../AddressData";

export default defineComponent({
  name: "AddressPicker",
  props: { modelValue: String },
  emits: ["update:modelValue"],
  data() {
    return {
      AddressData,
      province: null,
      city: null,
      county: null,
      detail: "",
      localForceUpdate: true,
      validators: {
        required: (value: any) => !!value || "此项为必填！",
      },
    };
  },
  methods: {
    changeValue() {
      this.$emit("update:modelValue", this.fullAddress);
    },
    getSelectItems(index: number): { label: string; value: string }[] {
      let ret: { label: string; value: string }[] = [];
      Object.entries(AddressData[index]).forEach((element) => {
        ret.push({ label: element[1], value: element[0] });
      });
      return ret;
    },
    changeProvince() {
      this.changeValue();
      this.city = null;
      this.county = null;
    },
    changeCity() {
      this.changeValue();
      this.county = null;
    },
  },
  computed: {
    fullAddress(): String {
      return (
        AddressData[100000][this.province] +
        AddressData[this.province][this.city] +
        AddressData[this.city][this.county] +
        this.detail
      );
    },
  },
});
</script>

<style scoped>
#title {
  font-size: 13px;
}
</style>
