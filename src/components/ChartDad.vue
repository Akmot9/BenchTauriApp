<template>
  <div>
    <h1>System Statistics</h1>
    <chart v-if="statistics" :statistics="statistics"></chart>
    <div v-else>
      <p>Loading statistics...</p>
    </div>
  </div>
</template>

<script>
import Chart from './Chart.vue';
import { listen } from '@tauri-apps/api/event';

export default {
  components: {
    Chart
  },
  data() {
    return {
      statistics: null
    };
  },
  mounted() {
    this.initializeStatisticsListener();
  },
  methods: {
    initializeStatisticsListener() {
      listen('update_statistics', (event) => {
        console.log("Received event with data:", event.payload);
        if (event.payload) {
          try {
            // Directly assigning the payload to statistics as it is already an object
            this.statistics = event.payload;
            console.log('Statistics updated:', this.statistics);
          } catch (error) {
            console.error('Error handling statistics:', error);
          }
        }
      }).catch(error => {
        console.error('Failed to initialize statistics listener:', error);
      });
    }
  }
};
</script>