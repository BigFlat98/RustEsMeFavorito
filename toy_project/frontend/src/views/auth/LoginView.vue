<template>
  <div>
  </div>
</template>

<script>
import { ref, computed } from 'vue';

export default {
  name: '',
  components: {},
  setup() {
    // 반응형 상태
    const state = ref({});

    // 계산된 속성
    const computedProps = computed(() => {});

    // 메서드
    const methods = {};

    // 생애주기 훅
    // onMounted(() => {});
    // onUnmounted(() => {});

    return {
      state,
      computedProps,
      methods
    };
  }
};
</script>

<style scoped>
</style>