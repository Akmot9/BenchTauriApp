<template>
  <div>
    
    <canvas ref="chartCanvas"></canvas>
  </div>
</template>

<script>
import { Chart, registerables } from 'chart.js';
Chart.register(...registerables);
import 'chartjs-adapter-date-fns';

export default {
  props: {
    data: {
      type: Object,
      required: true
    },
    statKey: {
      type: String,
      required: true
    }
  },
  data() {
    return {
      chart: null
    };
  },
  mounted() {
    console.log("Component mounted, creating chart...");
    this.createChart();
  },
  watch: {
    data: {
      deep: true,
      handler() {
        console.log("Data changed, updating chart...");
        this.createChart();
      }
    }
  },
  methods: {
    createChart() {
      console.log("Starting to create chart with data: ", this.data);
      // Here we assume the data passed is in the form of data.entries
      const labels = this.data.entries.map(entry => new Date(entry.time * 1000));
      console.log("Labels prepared: ", labels);

      // Prepare datasets, assuming each entry in entries contains a stats array
      const datasets = [];
      if (this.data.entries.length > 0 && this.data.entries[0].stats.length > 0) {
        // Loop through each unique stat name (assuming first entry has all potential stats)
        this.data.entries[0].stats.forEach(stat => {
          console.log(`Processing stat for ${stat.name}`);
          const dataPoints = this.data.entries.map(entry => {
            const statDetail = entry.stats.find(s => s.name === stat.name);
            return statDetail ? {
              x: new Date(entry.time * 1000),
              y: statDetail[this.statKey]
            } : null;
          }).filter(point => point != null); // Filter out any nulls from missing stats

          datasets.push({
            label: stat.name,
            data: dataPoints,
            borderColor: this.getRandomColor(),
            fill: false
          });
        });
      }

      if (this.chart) {
        console.log("Destroying existing chart...");
        this.chart.destroy();
      }

      console.log("Creating new chart...");
      this.chart = new Chart(this.$refs.chartCanvas, {
        type: 'line',
        data: { labels, datasets },
        options: {
          scales: {
            x: {
              type: 'time',
              time: {
                unit: 'second'
              },
              title: {
                display: true,
                text: 'Time'
              }
            },
            y: {
              title: {
                display: true,
                text: this.statKey.toUpperCase()
              }
            }
          }
        }
      });
      console.log("Chart created!");
    },
    getRandomColor() {
      const letters = '0123456789ABCDEF';
      let color = '#';
      for (let i = 0; i < 6; i++) {
        color += letters[Math.floor(Math.random() * 16)];
      }
      console.log(`Generated color: ${color}`);
      return color;
    }
  }
};

</script>

<style scoped>
canvas {
  max-width: 1000px;
  max-height: 500px;
}
</style>
