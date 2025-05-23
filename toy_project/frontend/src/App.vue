<template>
  <div class="app-container">
    <div class="nav-container">
      <Nav />
    </div>
    <div class="view-container">
      <router-view/>
    </div>
    <div class="footer-container">
      <Footer />
    </div>
  </div>
</template>

<script>
import { ref, computed } from 'vue';
import Nav from './components/layout/Nav.vue';
import Footer from './components/layout/Footer.vue';


export default {
  name: '',
  components: {
    Nav,
    Footer
  },
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
<style>
.nav-container {
  background-color: #fff177;
  width: 100%;
}

.main-container {
  background-color: #f0a6a6;
  height: 300px;
  width: 100%;
}

.footer-container {
  background-color: #88ffdb;
  height: 300px;
  width: 100%;
}

.app-container {
}

body {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
  width: 100%;
}
h1 {
  margin: 0;
}
</style>
