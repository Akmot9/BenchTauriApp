<template>
    <div>
      <canvas id="statsChart"></canvas>
    </div>
  </template>
  
  <script>
import { Chart, registerables } from 'chart.js';
import 'chartjs-adapter-date-fns'; // Importe l'adaptateur

Chart.register(...registerables);

export default {
  props: {
    statistics: {
      type: Object,
      required: true
    }
  },
  data() {
    return {
      chart: null,
    };
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
    this.initChart();
  },
  methods: {
    initChart() {
      const ctx = document.getElementById('statsChart').getContext('2d');
      this.chart = new Chart(ctx, {
        type: 'line',
        data: {
          labels: [], // Les timestamps seront ajoutés ici
          datasets: [
            {
              label: 'CPU Usage (%)',
              backgroundColor: 'rgba(255, 99, 132, 0.2)',
              borderColor: 'rgba(255, 99, 132, 1)',
              data: [], // Les données CPU seront ajoutées ici
            },
            {
              label: 'Memory Usage (%)',
              backgroundColor: 'rgba(54, 162, 235, 0.2)',
              borderColor: 'rgba(54, 162, 235, 1)',
              data: [], // Les données Memory seront ajoutées ici
            }
          ]
        },
        options: {
          scales: {
            x: {
              type: 'time',
              time: {
                unit: 'second'
              }
            }
          }
        }
      });
    },
    updateChartData(statistics) {
        if (this.chart) {
            const newLabels = statistics.entries.map(entry => new Date(entry.time * 1000));
            const newCpuData = statistics.entries.map(entry => {
            const cpuStat = entry.stats.find(stat => stat.name === 'CPU');
            return cpuStat ? cpuStat.cpu : null;
            });
            const newMemData = statistics.entries.map(entry => {
            const memStat = entry.stats.find(stat => stat.name === 'Memory');
            return memStat ? memStat.mem : null;
            });

            requestAnimationFrame(() => {
            this.chart.data.labels = newLabels;
            this.chart.data.datasets[0].data = newCpuData;
            this.chart.data.datasets[1].data = newMemData;
            this.chart.update();
            });
        }
        }


  }
};
</script>

  