<template>
  <div>
    <h1>System Statistics</h1>
    <statistic-chart v-if="statistics" :data="statistics" statKey="cpu"></statistic-chart>
    <statistic-chart v-if="statistics" :data="statistics" statKey="mem"></statistic-chart>
    <statistic-chart v-if="statistics" :data="statistics" statKey="vsz"></statistic-chart>

    <div v-else>
      <p>Loading statistics...</p>
    </div>
  </div>
</template>

<script>
import StatisticChart from './StatisticChart.vue';
import { listen } from '@tauri-apps/api/event';

export default {
  components: {
    StatisticChart
  },
  data() {
    return {
      statistics: null,
      unlistenEvent: null  // This will hold the unlisten function
    };
  },
  mounted() {
    this.initializeStatisticsListener();
  },
  beforeDestroy() {
    this.cleanupListeners();  // Clean up the listener when the component is destroyed
  },
  methods: {
    async initializeStatisticsListener() {
      // The listen function returns a Promise that resolves to the unlisten function
      this.unlistenEvent = await listen('update_statistics', (event) => {
        this.handleStatisticsUpdate(event.payload);
      });
    },
    handleStatisticsUpdate(payload) {
      if (payload) {
        this.statistics = payload;
      }
    },
    async cleanupListeners() {
      if (this.unlistenEvent) {
        await this.unlistenEvent();  // Call the unlisten function to remove the event listener
        console.log('Event listener cleaned up successfully.');
      }
    }
  }
};
</script>
