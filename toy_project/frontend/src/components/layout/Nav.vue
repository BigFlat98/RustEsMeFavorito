<template>
  <nav class="navbar navbar-expand-lg custom-navbar shadow-sm">
    <div class="container-fluid">
      <a class="navbar-brand custom-brand" href="#">쌈뽕네비</a>
      <button
        class="navbar-toggler"
        type="button"
        data-bs-toggle="collapse"
        data-bs-target="#navbarNav"
        aria-controls="navbarNav"
        aria-expanded="false"
        aria-label="Toggle navigation"
      >
        <span class="navbar-toggler-icon"></span>
      </button>
      <div class="collapse navbar-collapse" id="navbarNav">
        <ul class="navbar-nav ms-auto">
          <li class="nav-item">
            <a class="nav-link active" aria-current="page" href="#">
              <i class="fas fa-home"></i> 홈
            </a>
          </li>
          <li class="nav-item">
            <a class="nav-link" href="#">
              <i class="bi bi-gear"></i> 기능1
            </a>
          </li>
          <li class="nav-item">
            <a class="nav-link" href="#">
              <i class="bi bi-lightning"></i> 기능2
            </a>
          </li>
          <li class="nav-item">
            <a class="nav-link" href="#">
              <i class="bi bi-box-arrow-in-right"></i> 로그인
            </a>
          </li>
        </ul>
      </div>
    </div>
  </nav>
</template>

<script>
import { ref, computed } from 'vue';

export default {
  name: 'Nav',
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
.custom-navbar {
  background: #fff !important;
  border-bottom: 1px solid #eaeaea;
  min-height: 70px;
}

.custom-brand {
  color: #2d3a4b !important;
  font-weight: bold;
  letter-spacing: 1px;
}

.navbar-nav .nav-link {
  color: #2d3a4b !important;
  font-weight: 500;
  transition: color 0.2s, border 0.2s, background 0.2s;
  border-radius: 4px;
  margin-left: 4px;
  margin-right: 4px;
  padding: 8px 14px;
  border: 1px solid #2d3a4b;
  background: transparent !important;
  min-width: 90px;
  text-align: center;
}

.navbar-nav .nav-link.active {
  color: #2d3a4b !important;
  background: transparent !important;
  border: 1px solid #2d3a4b;
}

.navbar-nav .nav-link:hover {
  color: #fff !important;
  background: #2d3a4b !important;
  border: 1px solid #2d3a4b;
}
</style>