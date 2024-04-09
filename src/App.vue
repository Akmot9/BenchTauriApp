<template>
  <div id="app">
    <Chart />
    <BinaryList />
  </div>
</template>

<script>
import BinaryList from './components/BinaryList.vue';
import Chart from './components/Chart.vue';
export default {
  name: 'App',
  components: {
    BinaryList,
    Chart
  }
}
</script>
