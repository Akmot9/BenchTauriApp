<template>
  <div id="app">
    <ChartDad />
    <BinaryList />
  </div>
</template>

<script>
import BinaryList from './components/BinaryList.vue';
import ChartDad from './components/ChartDad.vue';
export default {
  name: 'App',
  components: {
    BinaryList,
    ChartDad
  }
}
</script>
