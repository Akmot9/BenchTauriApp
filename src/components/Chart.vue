<template>
    <div>
      <div>{{ stats }}</div>
      <canvas id="systemUsageChart"></canvas>
    </div>
  </template>
  
  <script>
  import { Chart, registerables } from 'chart.js';
  
  export default {
    name: 'SystemUsageChart',
    data() {
      return {
        stats: null,
        stat: null,
        chart: null,
      };
    },
    props: {
    statistics: {
      type: Object,
      required: true
    }
    },
    watch: {
    statistics: {
      handler(newVal) {
        this.updateChartData(newVal);
      },
      deep: true
    }
    },
    mounted() {
      Chart.register(...registerables);
  
      const ctx = document.getElementById('systemUsageChart').getContext('2d');
      this.chart = new Chart(ctx, {
        type: 'line',
        data: {
          labels: ['10:00', '10:05', '10:10', '10:15', '10:20', '10:25'], // Times of measurement
          datasets: [
            {
              label: 'Memory Usage (MB)',
              data: [120, 130, 125, 135, 140, 145], // Memory usage in MB
              fill: false,
              borderColor: 'rgb(75, 192, 192)',
              tension: 0.1
            },
            {
              label: 'CPU Usage (%)',
              data: [20, 25, 15, 30, 20, 45], // CPU usage in percentage
              fill: false,
              borderColor: 'rgb(255, 99, 132)',
              tension: 0.1
            }
          ]
        },
        options: {
          scales: {
            y: {
              beginAtZero: false, // Start from the lowest data value
              title: {
                display: true,
                text: 'Usage'
              }
            },
            x: {
              title: {
                display: true,
                text: 'Time'
              }
            }
          },
          responsive: true,
          maintainAspectRatio: false
        }
      });
    },
    methods: {
        updateChartData(statistics){
            console.log("stats in char",statistics);
            this.stats = statistics
            this.chart.data.labels +=1
            this.chart.data.labels +=1
        }
    },
    beforeDestroy() {
      if (this.chart) {
        this.chart.destroy();
      }
    }
  }
  </script>
  